use std::num::Wrapping;

use crate::js::update_dom_number;

#[derive(Debug)]
/// c - Carry flag
/// h - Half-Carry flag
/// o - Overflow flag
/// z - Zero flag
/// To view a more detailed description of each flag, look instead directly for it as a property inside this struct.
pub struct Flags {
    /// C (carry) flag:
    /// set to true if unsigned addition overflowed
    /// set to false if there was a borrow in unsigned subtraction
    pub c: bool,
    /// H (half-carry) flag:
    /// set to true if unsigned addition overflowed out of 4 least significant bits
    /// set to false if there was a borrow from the 5th bit in unsigned subtraction
    pub h: bool,
    /// O (overflow) flag:
    /// set if signed addition or subtraction overflowed (7th bits overflowed to the 8th one)
    /// set to false otherwise
    pub o: bool,
    /// Z (zero) flag:
    /// is set if result of addition is zero
    /// set to false otherwise
    pub z: bool,
}

impl Flags {
    pub fn new() -> Self {
        update_dom_number("FLAGS", 0, 4);
        Self {
            c: false,
            h: false,
            o: false,
            z: false,
        }
    }
    pub fn to_byte(&self) -> u8 {
        let c = self.c as u8;
        let h = self.h as u8;
        let o = self.o as u8;
        let z = self.z as u8;

        !c | !h << 1 | o << 2 | z << 3
    }
}

/// Arithmetic Logic Unit.
pub struct ALU {
    pub flags: Flags,
    pub res: u8,
}

impl ALU {
    pub fn new() -> Self {
        Self {
            flags: Flags::new(),
            res: 0,
        }
    }
}

impl ALU {
    pub fn not(&mut self, b: u8) -> u8 {
        !b
    }
    pub fn nor(&mut self, a: u8, b: u8) -> u8 {
        !(a | b)
    }
    pub fn nand(&mut self, a: u8, b: u8) -> u8 {
        !(a & b)
    }
    pub fn xor(&mut self, a: u8, b: u8) -> u8 {
        a ^ b
    }
    pub fn xnor(&mut self, a: u8, b: u8) -> u8 {
        !(a ^ b)
    }
    pub fn and(&mut self, a: u8, b: u8) -> u8 {
        a & b
    }
    pub fn or(&mut self, a: u8, b: u8) -> u8 {
        a | b
    }

    pub fn add(&mut self, a: u8, b: u8) -> u8 {
        let sum: u8 = (Wrapping(a) + Wrapping(b)).0;

        self.flags.c = a as u16 + b as u16 > sum as u16;
        self.flags.h = (a | b) & 1 << 4 != sum & 1 << 4;
        self.flags.o = (a | b) & 1 << 7 != sum & 1 << 7;
        self.flags.z = sum == 0;

        sum
    }
    pub fn adc(&mut self, a: u8, b: u8) -> u8 {
        let a = (Wrapping(a) + Wrapping(self.flags.c as u8)).0;
        self.add(a, b)
    }

    pub fn sub(&mut self, a: u8, b: u8) -> u8 {
        let negative_b = (Wrapping(!b) + Wrapping(1u8)).0;
        let diff = self.add(a, negative_b);

        self.flags.c = a > b;

        // half-carry flag is the opposite of borrow of the 5th bit
        self.flags.h = (a ^ b) & 1 << 4 == diff & 1 << 4;
        self.flags.o = (a ^ b) & 1 << 7 != diff & 1 << 7;
        diff
    }
    pub fn sbc(&mut self, a: u8, b: u8) -> u8 {
        // if the borrow (!carry) flag is set then add 1
        let a = (Wrapping(a) + Wrapping(!self.flags.c as u8)).0;
        self.sub(a, b)
    }

    /// normal subtraction but only update flags
    pub fn cmp(&mut self, a: u8, b: u8) {
        self.sub(a, b);
    }
    pub fn inc(&mut self, b: u8) -> u8 {
        self.add(b, 1)
    }
    pub fn dec(&mut self, b: u8) -> u8 {
        self.sub(b, 1)
    }
    /// will set flags to the values of the nth bits of the result
    pub fn shl(&mut self, b: u8) -> u8 {
        let res: u8 = b << 1;

        self.flags.c = b & 0b1000_0000 == 1;
        self.flags.h = b & 0b0000_0100 == 1;
        self.flags.o = b & 0b0100_0000 == 1;
        self.flags.z = res == 0;

        res
    }
}

#[cfg(test)]
mod test_adding {
    use super::*;

    #[test]
    fn test_no_flags() {
        let mut alu = ALU::new();
        assert_eq!(alu.add(2, 2), 4);
        assert_eq!(alu.add(128, 128), 0);
    }
    #[test]
    fn test_flag_c() {
        let mut alu = ALU::new();
        assert_eq!(alu.add(2, 2), 4);
        assert_eq!(alu.flags.c, false);

        assert_eq!(alu.add(255, 1), 0);
        assert_eq!(alu.flags.c, true);

        assert_eq!(alu.add(255, 255), 254);
        assert_eq!(alu.flags.c, true);
    }
    #[test]
    fn test_flag_h() {
        let mut alu = ALU::new();

        assert_eq!(alu.add(8, 8), 16);
        assert_eq!(alu.flags.h, true);

        assert_eq!(alu.add(0b1111_1111, 1), 0);
        assert_eq!(alu.flags.h, true);

        assert_eq!(alu.add(0b1111_0000, 0b1111), 255);
        assert_eq!(alu.flags.h, false);
    }
    #[test]
    fn test_flag_o() {
        let mut alu = ALU::new();

        assert_eq!(alu.add(0b1000_0000, 0b1000_0000), 0);
        assert_eq!(alu.flags.o, true);

        assert_eq!(alu.add(0b0111_1111, 0b0000_0001), 128);
        assert_eq!(alu.flags.o, true);

        assert_eq!(alu.add(0b1000_0000, 0b0000_0000), 128);
        assert_eq!(alu.flags.o, false);
    }
    #[test]
    fn test_flag_z() {
        let mut alu = ALU::new();

        assert_eq!(alu.add(0, 0), 0);
        assert_eq!(alu.flags.z, true);

        assert_eq!(alu.add(255, 1), 0);
        assert_eq!(alu.flags.z, true);
    }
    #[test]
    fn test_add_with_carry() {
        let mut alu = ALU::new();

        alu.flags.c = true;
        assert_eq!(alu.adc(0, 0), 1);

        alu.flags.c = true;
        assert_eq!(alu.adc(255, 0), 0);

        alu.flags.c = true;
        assert_eq!(alu.adc(255, 1), 1);
    }
}
#[cfg(test)]
mod test_subtracting {
    use super::*;
    #[test]
    fn test_no_flags() {
        let mut alu = ALU::new();

        assert_eq!(alu.sub(4, 2), 2);
        assert_eq!(alu.sub(0, 1), 255);
    }
    #[test]
    fn test_flag_c() {
        let mut alu = ALU::new();
        // carry flag is the opposite of borrow

        assert_eq!(alu.sub(0, 1), 255);
        assert_eq!(alu.flags.c, false);

        assert_eq!(alu.sub(0, 255), 1);
        assert_eq!(alu.flags.c, false);

        assert_eq!(alu.sub(1, 0), 1);
        assert_eq!(alu.flags.c, true);
    }
    #[test]
    fn test_flag_h() {
        let mut alu = ALU::new();

        assert_eq!(alu.sub(0, 1), 255);
        assert_eq!(alu.flags.h, false);

        assert_eq!(alu.sub(0, 255), 1);
        assert_eq!(alu.flags.h, false);

        assert_eq!(alu.sub(1, 0), 1);
        assert_eq!(alu.flags.h, true);
    }
    #[test]
    fn test_flag_o() {
        let mut alu = ALU::new();

        assert_eq!(alu.sub(0, 1), 255);
        assert_eq!(alu.flags.o, true);

        assert_eq!(alu.sub(1, 0), 1);
        assert_eq!(alu.flags.o, false);

        assert_eq!(alu.sub(128, 128), 0);
        assert_eq!(alu.flags.o, false);
    }
    #[test]
    fn test_flag_z() {
        let mut alu = ALU::new();

        assert_eq!(alu.sub(0, 0), 0);
        assert_eq!(alu.flags.z, true);

        assert_eq!(alu.sub(1, 1), 0);
        assert_eq!(alu.flags.z, true);

        assert_eq!(alu.sub(0, 1), 255);
        assert_eq!(alu.flags.z, false);
    }

    #[test]
    fn test_subtract_with_borrow() {
        let mut alu = ALU::new();

        alu.flags.c = true;
        assert_eq!(alu.sbc(0, 0), 0);

        alu.flags.c = false;
        assert_eq!(alu.sbc(0, 0), 1);

        alu.flags.c = true;
        assert_eq!(alu.sbc(255, 255), 0);

        alu.flags.c = false;
        assert_eq!(alu.sbc(255, 255), 1);
    }
}
