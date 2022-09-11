use crate::alu::*;
use crate::clock::*;
use crate::lcd::*;
use crate::memory::*;
use crate::microcode::*;
use crate::pc::*;
use crate::reg::*;

use serde::{Deserialize, Serialize};
use std::num::Wrapping;
use wasm_bindgen::prelude::*;

// import console.log as console_log!
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}
macro_rules! console_log {
     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Debug)]
/// Contains all the modules required for
/// the cpu to run.
// ok this looks REALLY ugly but
// at the moment i dont know any other
// way to have public elements yet make wasm
// not create gettters and setters for it
pub struct Cpu {
    #[wasm_bindgen(skip)]
    pub clk: Clock,
    #[wasm_bindgen(skip)]
    pub ram: Ram,
    #[wasm_bindgen(skip)]
    pub rom: Rom,
    #[wasm_bindgen(skip)]
    pub mar: [Register; 2],
    #[wasm_bindgen(skip)]
    pub pc: ProgramCounter,
    #[wasm_bindgen(skip)]
    pub sp: Register,
    #[wasm_bindgen(skip)]
    pub registers: [Register; 4],
    #[wasm_bindgen(skip)]
    pub alu: Alu,
    #[wasm_bindgen(skip)]
    pub lcd: Lcd,
    #[wasm_bindgen(skip)]
    pub ir: Register,
    #[wasm_bindgen(skip)]
    pub decoder: Microcode,
    #[wasm_bindgen(skip)]
    pub control_word: u32,
}

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Self {
        console_log!("Hi there, this is the CBT speaking.");
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
            ir: Register::new(),
            alu: Alu::new(),
            decoder: Microcode::new(),
            lcd: Lcd::new(),
            control_word: 0,
        }
    }
    pub fn rst(&mut self) {
        self.clk.state = false;
        self.ram.rst();
        self.mar[0].set(0);
        self.mar[1].set(0);
        self.pc.set(0);
        self.alu.set_flag('c', false);
        self.alu.set_flag('n', false);
        self.alu.set_flag('o', false);
        self.alu.set_flag('z', false);
    }
    pub fn get_lcd(&mut self) {
        console_log!("{}", self.lcd.display.buffer.get(0).unwrap().to_string());
    }
    pub fn fetch(&mut self) {
        self.ir
            .set(Memory::get(&self.rom, &self.ram, self.pc.get()));
        self.pc.increment();
    }
    /// 1. Rising edge of the clock.
    /// 2. Fetch instruction.
    /// 3. Decode it.
    /// 4. Execute it.
    /// 5. Falling edge of the clock.
    pub fn cycle(&mut self) {
        self.clk.tick();
        self.fetch();
        let mut decoder = self.decoder;
        decoder.decode_and_exectue(self);
        self.clk.tick();
    }
}
