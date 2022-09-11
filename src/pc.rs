//! Program counter.

pub use crate::bus::AddressBus;
use serde::{Deserialize, Serialize};
use std::num::Wrapping;

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
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
