use crate::alu::ALU;
use crate::bus::Bus;
use crate::cw::*;

use crate::memory::Memory;
use crate::microcode::Microcode;
use crate::pc::ProgramCounter;
use crate::reg::Register;

pub struct Clock {
    pub utime: u8,
    pub halted: bool,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            utime: 0,
            halted: false,
        }
    }
    /// utime will overflow if it reaches 16
    /// setting SR bit is preffered way of resetting it
    pub fn tick(&mut self) {
        self.utime += 1;
        self.utime &= 0b1111;
    }
    pub fn rst(&mut self) {
        self.utime = 0;
    }
    pub fn hlt(&mut self) {
        self.halted = true;
    }
}

pub struct Cpu {
    pub bus: Bus,
    clock: Clock,
    ucode: Microcode,
    mem: Memory,
    ir: Register,
    pc: ProgramCounter,

    ra: Register,
    rb: Register,
    rc: Register,
    rd: Register,
    sp: Register,
    alu: ALU,
    //     lcd: Lcd,
}

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
        }
    }
    pub fn load_program(&mut self, prg: Vec<u8>) {
        self.mem = Memory::new(prg);
    }

    fn cycle(&mut self) {
        let cw =
            self.ucode
                .instruction_to_cw(self.ir.0, self.alu.flags.to_byte(), self.clock.utime);

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
        self.bus.0 = bus;
    }
    pub fn tick(&mut self) {
        self.clock.tick();
        self.cycle();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetching() {
        let mut cpu = Cpu::new();
        cpu.cycle();
        cpu.clock.tick();
        // first cycle is always LPO | LAI
        // therefore after that cycle we should have
        // lower 8 bits of program counter in mem address register
        // assert_eq!(cpu.mem.address, )
    }
}
