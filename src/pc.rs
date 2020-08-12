pub use crate::bus::AddressBus;
use std::num::Wrapping;

pub struct ProgramCounter {
    value: Wrapping<u16>,
}

impl ProgramCounter {
    pub fn new() -> Self {
        Self { value: Wrapping(0) }
    }
    pub fn increment(&mut self) {
        self.value += Wrapping(1);
    }
}

impl AddressBus for ProgramCounter {
    fn get(&self) -> Wrapping<u16> {
        self.value
    }
    fn set(&mut self, value: u16) {
        self.value = Wrapping(value);
    }
}
