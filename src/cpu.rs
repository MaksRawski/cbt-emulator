use wasm_bindgen::prelude::wasm_bindgen;

use crate::alu::ALU;
use crate::bus::Bus;
use crate::clock::Clock;
use crate::{console_log, cw::*};

use crate::js::{log, update_cw, update_dom_number};
use crate::lcd::Lcd;
use crate::memory::Memory;
use crate::microcode::Microcode;
use crate::pc::ProgramCounter;
use crate::reg::Register;

#[wasm_bindgen]
pub struct Cpu {
    pub bus: Bus,
    pub clock: Clock,
    pub pc: ProgramCounter,
    pub lcd: Lcd,
    alu: ALU,
    mem: Memory,
    ucode: Microcode,

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
}

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Self {
        update_cw(0);
        update_dom_number("BUS", 0, 8);
        Self {
            bus: Bus(0),
            clock: Clock::new(),
            ucode: Microcode::load(),
            ir: Register::new("IR"),
            mem: Memory::new(vec![]),
            pc: ProgramCounter::new(),

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
        let cw = self.ucode.instruction_to_cw(
            &self.ir.data,
            &self.alu.flags.to_byte(),
            &self.clock.utime,
        );
        update_cw(cw);

        let bus = match cw {
            cw if (cw & AO > 0) => self.ra.o(),
            cw if (cw & BO > 0) => self.rb.o(),
            cw if (cw & CO > 0) => self.rc.o(),
            cw if (cw & DO > 0) => self.rd.o(),
            cw if (cw & SPO > 0) => self.sp.o(),

            cw if (cw & MO > 0) => self.mem.o(),
            cw if (cw & LPO > 0) => self.pc.lo(),
            cw if (cw & HPO > 0) => self.pc.ho(),
            cw if (cw & ALO > 0) => self.alu.res,
            _ => 0,
        };

        self.bus.0 = bus.clone();
        update_dom_number("BUS", self.bus.0.into(), 8);

        for i in 0..32 {
            match cw & 1 << i {
                HLT => self.clock.hlt(),

                LAI => self.mem.lai(bus),
                HAI => self.mem.hai(bus),
                MI => self.mem.i(bus),

                II => self.ir.i(bus),
                SR => self.clock.rst(),

                PCC => self.pc.c(),
                LPI => self.pc.li(bus),
                HPI => self.pc.hi(bus),

                AI => self.ra.i(bus),
                BI => self.rb.i(bus),
                CI => self.rc.i(bus),
                DI => self.rd.i(bus),
                SPI => self.sp.i(bus),
                _ => {}
            }
        }
        if cw & ALE > 0 {
            if (self.ir.data & 0b00111100) >> 2 == 0b1100 {
                self.alu.cmp(bus, self.ra.data);
                unsafe { console_log!("ALU {:?}", self.alu.flags) }
                unsafe { console_log!("ALU flags to_byte(): {:?}", self.alu.flags.to_byte()) }
            }
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
                SUB_A_B => self.alu.sub(bus, self.ra.data),
                SBC_A_B => self.alu.sbc(bus, self.ra.data),
                INC_A => self.alu.inc(bus),
                DEC_A => self.alu.dec(bus),
                SHL_A => self.alu.shl(bus),

                _ => {
                    // ALU used as temporary memory
                    if cw & (ALE | ALM) > 0 {
                        bus
                    } else {
                        dbg!("else");
                        self.alu.res
                    }
                }
            };
            // in the interface display flags normally,
            // not how microcode wants it
            update_dom_number("ALU", self.alu.res.into(), 8);
            update_dom_number("FLAGS", (self.alu.flags.to_byte() ^ 0b11).into(), 4);
        }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetching() {
        let mut cpu = Cpu::new();
        cpu.load_program(vec![42]);

        for i in 0..3 {
            let cw = cpu.ucode.instruction_to_cw(
                &cpu.ir.data,
                &cpu.alu.flags.to_byte(),
                &cpu.clock.utime,
            );

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

        let ram = cpu.mem.ram.0;

        assert_eq!(ram.get(0), Some(&42u8));
    }
    #[test]
    fn test_alu_ops() {
        let mut cpu = Cpu::new();

        // mov a, 42
        // inc a
        cpu.load_program(vec![0x07, 0x2a, 0xf4, 0x36]);
        for _ in 0..50 {
            cpu.tick()
        }
        assert_eq!(cpu.ra.o(), 43);
    }
}
