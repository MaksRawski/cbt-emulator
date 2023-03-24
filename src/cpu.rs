use wasm_bindgen::prelude::wasm_bindgen;

use crate::alu::ALU;
use crate::clock::Clock;
use crate::{cw::*, cw_match, cw_switch};

use crate::interrupts::ProgrammableInterruptController;
use crate::js::{update_cw, update_dom_number, update_flags};
use crate::lcd::Lcd;
use crate::memory::Memory;
use crate::microcode::Microcode;
use crate::pc::ProgramCounter;
use crate::reg::Register;

#[wasm_bindgen]
pub struct Cpu {
    clock: Clock,
    #[wasm_bindgen(skip)]
    pub pc: ProgramCounter,
    alu: ALU,
    mem: Memory,
    ucode: Microcode,
    pic: ProgrammableInterruptController,

    #[wasm_bindgen(skip)]
    pub ir: Register,
    #[wasm_bindgen(skip)]
    pub ra: Register,
    #[wasm_bindgen(skip)]
    pub rb: Register,
    #[wasm_bindgen(skip)]
    pub rc: Register,
    #[wasm_bindgen(skip)]
    pub rd: Register,
    #[wasm_bindgen(skip)]
    pub sp: Register,

    #[wasm_bindgen(skip)]
    pub lcd: Lcd,
}

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Self {
        update_cw(0);
        update_dom_number("BUS", 0, 8);
        Self {
            clock: Clock::new(),
            ucode: Microcode::load(),
            ir: Register::new("IR"),
            mem: Memory::new(vec![]),
            pc: ProgramCounter::new(),
            pic: ProgrammableInterruptController::new(),

            ra: Register::new("RA"),
            rb: Register::new("RB"),
            rc: Register::new("RC"),
            rd: Register::new("RD"),
            sp: Register::new("SP"),

            alu: ALU::new(),
            lcd: Lcd::new(),
        }
    }
    pub fn load_program(&mut self, prg: Vec<u8>) {
        self.mem = Memory::new(prg);
    }

    fn cycle(&mut self) {
        let cw =
            self.ucode
                .instruction_to_cw(&self.ir.data, &self.alu.flags.o(), &self.clock.utime);
        update_cw(cw);

        // these control bits only do their job when they are the only one set
        if cw == HLT {
            self.clock.hlt();
            return;
        } else if cw == SR {
            self.clock.rst();
            return;
        }

        // there are pull up resistors on the real thing so
        // if nothing is outputing then all the bits are set
        let mut bus = cw_match!(cw, 0xff,
            AO => self.ra.o(),
            BO => self.rb.o(),
            CO => self.rc.o(),
            DO => self.rd.o(),
            MO => self.mem.o(),
            SPO => self.sp.o(),
            LPO => self.pc.lo(),
            HPO => self.pc.ho(),
            ALO => self.alu.res
        );

        // see microcode documentation for these ~ridicolous~ special control words
        if cw == (HLT | HPI) {
            bus = self.pic.handle();
        } else if cw == (HLT | ALO) {
            bus = self.alu.flags.o();
        }

        update_dom_number("BUS", bus.into(), 8);

        cw_switch!(cw,
            AI => self.ra.i(bus),
            BI => self.rb.i(bus),
            CI => self.rc.i(bus),
            DI => self.rd.i(bus),
            SPI => self.sp.i(bus),
            PCC => self.pc.c(),
            LPI => self.pc.li(bus),
            HPI => self.pc.hi(bus),
            LAI => self.mem.lai(bus),
            HAI => self.mem.hai(bus),
            MI => self.mem.i(bus),
            II => self.ir.i(bus)
        );

        // ALU operations
        if cw & ALE > 0 {
            let alu_cw = cw & (ALM | ALE | ALO | AL0 | AL1 | AL2 | AL3 | ALC);
            self.alu.res = match alu_cw {
                NOT_A => self.alu.not(bus),
                A_NOR_B => self.alu.nor(bus, self.ra.data),
                A_NAND_B => self.alu.nand(bus, self.ra.data),
                NOT_B => self.alu.not(self.ra.data),
                A_XOR_B => self.alu.xor(bus, self.ra.data),
                A_XNOR_B => self.alu.xnor(bus, self.ra.data),
                A_AND_B => self.alu.and(bus, self.ra.data),
                A_OR_B => self.alu.or(bus, self.ra.data),

                ADD_A_B => self.alu.add(bus, self.ra.data),
                ADC_A_B => self.alu.adc(bus, self.ra.data),
                // SUB and CMP have the same cw and their only difference is that in the next tick CMP
                // won't do anything with the result, but that's already handled in the microcode
                SUB_A_B => self.alu.sub(bus, self.ra.data),
                SBC_A_B => self.alu.sbc(bus, self.ra.data),
                INC_A => self.alu.inc(bus),
                DEC_A => self.alu.dec(bus),
                SHL_A => self.alu.shl(bus),

                // if ALU is enabled, but not given any valid cw
                // it just stores whatever is on the bus
                _ => bus,
            };

            update_dom_number("ALU", self.alu.res.into(), 8);
            update_flags(&self.alu.flags).unwrap();
        }

        // LCD
        if cw & LCE > 0 && cw & LCM > 0 {
            self.lcd.cmd(bus);
        } else if (cw & LCE) > 0 {
            self.lcd.txt(bus);
        }
    }
    pub fn tick(&mut self) {
        self.cycle();
        self.clock.tick();
    }
    pub fn view_rom(&self) -> Vec<u8> {
        self.mem.view_rom().to_vec()
    }
    pub fn request_interrupt(&mut self, _device_id: u8) {}
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetching() {
        let mut cpu = Cpu::new();
        cpu.load_program(vec![42]);

        for i in 0..3 {
            let cw =
                cpu.ucode
                    .instruction_to_cw(&cpu.ir.data, &cpu.alu.flags.o(), &cpu.clock.utime);

            match i {
                0 => assert_eq!(cw, LPO | LAI),
                1 => assert_eq!(cw, HPO | HAI),
                2 => assert_eq!(cw, PCC | MO | II),
                _ => panic!(),
            }

            cpu.tick()
        }
        assert_eq!(cpu.ir.data, 42);
    }

    #[test]
    fn test_pc_in_cpu() {
        let mut cpu = Cpu::new();
        // nop - mov a, a - 4 steps
        cpu.load_program(vec![0]);
        for _ in 0..4 {
            cpu.tick();
        }
        assert_eq!(cpu.pc.lo(), 1);
    }

    #[test]
    fn test_stores() {
        let mut cpu = Cpu::new();

        // mov a, 42
        // store [0x8000], a
        // hlt
        cpu.load_program(vec![0x07, 0x2a, 0xb8, 0x80, 0x00, 0x36]);
        for _ in 0..60 {
            cpu.tick();
        }

        cpu.mem.lai(0);
        cpu.mem.hai(0x80);

        assert_eq!(cpu.mem.o(), 42);
    }
    #[test]
    fn test_alu_ops() {
        let mut cpu = Cpu::new();

        // mov a, 42
        // inc a
        cpu.load_program(vec![0x07, 0x2a, 0xf4, 0x36]);
        for _ in 0..100 {
            cpu.tick()
        }
        dbg!(cpu.alu.res);
        assert_eq!(cpu.ra.o(), 43);
    }
}
