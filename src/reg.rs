//! 8 Bit Register.
use wasm_bindgen::prelude::wasm_bindgen;

pub use crate::bus::Bus;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Register(pub u8);

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Register {
    pub fn new() -> Register {
        Register(0)
    }
    pub fn o(&self) -> u8 {
        self.0
    }
    pub fn i(&mut self, v: u8) {
        self.0 = v;
    }
}
