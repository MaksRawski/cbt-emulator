use wasm_bindgen::prelude::wasm_bindgen;

use crate::alu::ALU;
use crate::bus::Bus;
use crate::clock::Clock;
use crate::cw::*;

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
    pub ir: Register,
    mem: Memory,
    ucode: Microcode,

    ra: Register,
    rb: Register,
    rc: Register,
    rd: Register,
    sp: Register,
    alu: ALU,
    pub lcd: Lcd,
}

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Self {
        Self {
            bus: Bus(0),
            clock: Clock::new(),
            ucode: Microcode::load(),
            ir: Register::new(),
            mem: Memory::new(vec![]),
            pc: ProgramCounter::new(),

            ra: Register::new(),
            rb: Register::new(),
            rc: Register::new(),
            rd: Register::new(),
            sp: Register::new(),

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
                .instruction_to_cw(&self.ir.0, &self.alu.flags.to_byte(), &self.clock.utime);

        let bus = match cw {
            cw if (cw & AO > 0) => self.ra.o(),
            cw if (cw & BO > 0) => self.rb.o(),
            cw if (cw & CO > 0) => self.rc.o(),
            cw if (cw & DO > 0) => self.rd.o(),
            cw if (cw & SPO > 0) => self.sp.o(),

            // cw if (cw &   > 0) => self.sp.o(),
            cw if (cw & MO > 0) => self.mem.o(),
            cw if (cw & LPO > 0) => self.pc.lo(),
            cw if (cw & HPO > 0) => self.pc.ho(),
            _ => 0,
        };

        for i in 0..16 {
            match cw & 1 << i {
                HLT => self.clock.hlt(),

                LAI => self.mem.lai(bus),
                HAI => self.mem.hai(bus),
                MI => self.mem.i(bus).unwrap(), // TODO properly handle instead of just unwrapping

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
        if (cw & LCE | LCM) > 0 {
            self.lcd.cmd(bus);
        } else if (cw & LCE) > 0 {
            self.lcd.txt(bus);
        }
        self.bus.0 = bus;
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
    fn test_lcd() {
        let mut cpu = Cpu::new();
        cpu.load_program(vec![42]);
        assert_eq!(cpu.view_rom(), vec![42]);
    }
}
