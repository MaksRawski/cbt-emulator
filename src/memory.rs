//! Includes definitions of ROM and RAM.
//!
//! CBT uses Harvard architecture (kinda)
//! so ROM is the only place from which instructions
//! can be fetched, while RAM can be used by programs.

//! Stack is placed at last 255 bytes of RAM.
//! Though accessing it is all up to user.
//! (put 127 into high address register)

use crate::js::update_dom_number;

pub const RAM_SIZE: u16 = 32768;
pub const ROM_SIZE: u16 = 32768;

pub struct Memory {
    pub ram: Ram,
    pub rom: Rom,
    pub address: u16,
}

impl Memory {
    pub fn new(rom_data: Vec<u8>) -> Self {
        Self {
            ram: Ram::new(),
            rom: Rom::new(rom_data),
            address: 0,
        }
    }
    /// will return 0 if a given address wasn't used before
    pub fn o(&self) -> u8 {
        if self.address < ROM_SIZE {
            *self.rom.0.get(self.address as usize).unwrap_or(&0u8)
        } else {
            *self
                .ram
                .0
                .get(self.address as usize - 0x8000)
                .unwrap_or(&0u8)
        }
    }

    pub fn i(&mut self, v: u8) {
        if self.address < ROM_SIZE {
            return;
        }
        self.ram.0[(self.address - ROM_SIZE) as usize] = v;
        update_dom_number("RAM", v.into(), 8);
    }
    /// low address in
    pub fn lai(&mut self, a: u8) {
        self.address = a as u16 | (self.address & 0b1111_1111 << 8);
        update_dom_number("RAM", self.o().into(), 8);
        update_dom_number("MAR", self.address, 16);
    }
    /// high address in
    pub fn hai(&mut self, a: u8) {
        self.address = (a as u16) << 8 | (self.address & 0b1111_1111);
        update_dom_number("RAM", self.o().into(), 8);
        update_dom_number("MAR", self.address, 16);
    }
    pub fn view_rom(&self) -> &Vec<u8> {
        &self.rom.0
    }
}

pub struct Ram(pub Vec<u8>);

pub struct Rom(Vec<u8>);

impl Ram {
    pub fn new() -> Self {
        update_dom_number("RAM", 0, 8);
        update_dom_number("MAR", 0, 16);
        Self(vec![0; RAM_SIZE.into()])
    }
}

impl Rom {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rom_addressing() {
        let mut mem = Memory::new(vec![1, 2, 34]);
        mem.hai(0);
        mem.lai(2);

        assert_eq!(mem.o(), 34);
    }
    #[test]
    fn test_ram_addressing() {
        let mut mem = Memory::new(vec![]);
        mem.ram.0[0] = 42;

        mem.hai(0x80);
        mem.lai(0);

        assert_eq!(mem.o(), 42);
    }
}
