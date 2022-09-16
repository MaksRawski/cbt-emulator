//! Program counter.

use std::num::Wrapping;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct ProgramCounter(u16);

#[wasm_bindgen]
impl ProgramCounter {
    pub fn new() -> Self {
        Self(0u16)
    }
    pub fn c(&mut self) {
        self.0 = (Wrapping(self.0) + Wrapping(1u16)).0;
    }
    pub fn lo(&self) -> u8 {
        (self.0 & 0b1111_1111) as u8
    }
    pub fn ho(&self) -> u8 {
        ((self.0 & 0b1111_1111 << 8) >> 8) as u8
    }

    pub fn li(&mut self, v: u8) {
        self.0 = v as u16 | (self.0 & 0b1111_1111 << 8);
    }
    pub fn hi(&mut self, v: u8) {
        self.0 = (v as u16) << 8 | self.0 & 0b1111_1111;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pc() {
        let mut pc = ProgramCounter::new();
        pc.li(24);
        pc.hi(42);

        assert_eq!(pc.lo(), 24);
        assert_eq!(pc.ho(), 42);
    }
    #[test]
    fn test_overflow() {
        let mut pc = ProgramCounter::new();
        pc.li(255);
        pc.hi(255);
        pc.c();

        assert_eq!(pc.lo(), 0);
        assert_eq!(pc.ho(), 0);
    }
}
