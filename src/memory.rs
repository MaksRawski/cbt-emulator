//! Includes definitions of ROM and RAM.
//!
//! CBT uses Harvard architecture (kinda)
//! so ROM is the only place from which instructions
//! can be fetched, while RAM can be used by programs.

//! Stack is placed at last 255 bytes of RAM.
//! Though accessing it is all up to user.
//! (put 127 into high address register)

pub const RAM_SIZE: u16 = 32768;
pub const ROM_SIZE: u16 = 32768;

pub struct Memory {
    ram: Ram,
    rom: Rom,
    address: u16,
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
            *self.ram.0.get(self.address as usize).unwrap_or(&0u8)
        }
    }

    /// returns err if tried to write into rom
    pub fn i(&mut self, v: u8) {
        if self.address < ROM_SIZE {
            return;
        }
        self.ram.0[(self.address - ROM_SIZE) as usize] = v;
    }
    /// low address in
    pub fn lai(&mut self, a: u8) {
        self.address = a as u16 | (self.address & 0b1111_1111 << 8);
    }
    /// high address in
    pub fn hai(&mut self, a: u8) {
        self.address = (a as u16) << 8 | (self.address & 0b1111_1111);
    }
    pub fn view_rom(&self) -> &Vec<u8> {
        &self.rom.0
    }
}

pub struct Ram(Vec<u8>);

pub struct Rom(Vec<u8>);

impl Ram {
    pub fn new() -> Self {
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
    fn test_addressing() {
        let mut mem = Memory::new(vec![42, 1, 23]);
        mem.lai(2);
        mem.hai(0);
        assert_eq!(mem.o(), 23);
    }
}
