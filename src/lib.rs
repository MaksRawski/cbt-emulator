pub mod alu;
pub mod bus;
pub mod memory;
pub mod pc;
pub mod reg;

use alu::*;
use memory::*;
use pc::*;
use reg::*;

/// Goal of this library is to emulate only
/// results of instructions but doing so
/// while keeping track of how many clock cycles
/// each instruction consumes on real hardware

pub struct Cpu {
    pub clk: Clock,
    pub ram: Ram,
    pub rom: Rom,
    pub mar: [Register; 2],
    pub pc: ProgramCounter,
    pub sp: Register,
    pub registers: [Register; 4],
    pub alu: Alu,
    pub decoder: Microcode,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            clk: Clock::new(),
            ram: Ram::new(),
            rom: Rom::new(),
            mar: [Register::new(), Register::new()],
            pc: ProgramCounter::new(),
            sp: Register::new(),
            registers: [
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
            ],
            alu: Alu::new(),
            decoder: Microcode::new(),
        }
    }
    pub fn rst() {}
}

pub struct Clock {
    pub state: bool,
}

impl Clock {
    pub fn new() -> Self {
        Self { state: false }
    }
    pub fn tick(&mut self) {
        self.state = !self.state;
    }
}

// decoding logic
pub struct Microcode {}
impl Microcode {
    pub fn new() -> Self {
        Self {}
    }
}
