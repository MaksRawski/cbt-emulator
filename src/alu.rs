//! Arithmetic logic unit.
use crate::bus::DataBus;
use crate::reg::Register;
use serde::{Deserialize, Serialize};

// #[cfg(test)]
// use quickcheck::{Arbitrary, Gen};
use std::num::Wrapping;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Alu {
    // only contains flags as its internal state
    flags: [bool; 4],
}

impl Alu {
    pub fn new() -> Self {
        let flags = [false; 4];
        Self { flags }
    }

    fn get_index(&self, flag: char) -> usize {
        match flag {
            'c' => 0,
            'n' => 1,
            'o' => 2,
            'z' => 3,
            _ => panic!("Tried to use non existent flag"),
        }
    }

    /// # Panics
    /// When trying to use non existent flag.
    pub fn set_flag(&mut self, flag: char, value: bool) {
        // 3     2          1        0
        // c     n          o        z
        // carry negtive    overflow zero
        self.flags[self.get_index(flag)] = value;
    }

    /// # Panics
    /// When trying to use non existent flag.
    pub fn get_flag(&self, flag: char) -> bool {
        self.flags[self.get_index(flag)]
    }
}

/// argument A needs to get &Register A
/// argument B is the chosen &Register
impl Alu {
    pub fn not(&mut self, b: &Register) -> Wrapping<u8> {
        !b.get()
    }
    pub fn nor(&mut self, a: &Register, b: &Register) -> Wrapping<u8> {
        !(a.get() | b.get())
    }
    pub fn nand(&mut self, a: &Register, b: &Register) -> Wrapping<u8> {
        !(a.get() & b.get())
    }
    pub fn xor(&mut self, a: &Register, b: &Register) -> Wrapping<u8> {
        a.get() ^ b.get()
    }
    pub fn xnor(&mut self, a: &Register, b: &Register) -> Wrapping<u8> {
        !(a.get() ^ b.get())
    }
    pub fn and(&mut self, a: &Register, b: &Register) -> Wrapping<u8> {
        a.get() & b.get()
    }
    pub fn or(&mut self, a: &Register, b: &Register) -> Wrapping<u8> {
        a.get() | b.get()
    }

    pub fn add(&mut self, a: &Register, b: &Register) -> Wrapping<u8> {
        let sum = a.get() + b.get();
        let non_wrapping_sum = (a.get().0) as u16 + (b.get().0) as u16;

        // overflow flag is set when signed addition overflows
        // that happens when msb of a and msb of b are the same
        // but msb of result is diffrent
        let mut overflow = false;
        let msb_a = a.get().0 >> 7;
        let msb_b = b.get().0 >> 7;
        if (msb_a == msb_b) && ((msb_a & msb_b) != sum.0 >> 7) {
            overflow = true;
        }

        self.set_flag('c', non_wrapping_sum > 255);
        self.set_flag('n', sum & Wrapping(128) == Wrapping(128));
        self.set_flag('o', overflow);
        self.set_flag('z', sum == Wrapping(0u8));

        sum
    }
    pub fn adc(&mut self, a: &Register, b: &Register) -> Wrapping<u8> {
        let sum = a.get() + b.get() + Wrapping(self.get_flag('c') as u8);
        let non_wrapping_sum = (a.get().0) as u16 + (b.get().0) as u16 + self.get_flag('c') as u16;

        // overflow flag is set when signed addition overflows
        // that happens when msb of a and msb of b are the same
        // but msb of result is diffrent
        let mut overflow = false;
        let msb_a = a.get().0 + self.get_flag('c') as u8 & 1 << 7;
        let msb_b = b.get().0 >> 7;
        if (msb_a == msb_b) && ((msb_a & msb_b) != sum.0 & 1 << 7) {
            overflow = true;
        }

        self.set_flag('c', non_wrapping_sum > 255);
        self.set_flag('n', sum & Wrapping(128) == Wrapping(128));
        self.set_flag('o', overflow);
        self.set_flag('z', sum == Wrapping(0u8));

        sum
    }
    pub fn sub(&mut self, a: &Register, b: &Register) -> Wrapping<u8> {
        let diffrence = b.get() - a.get();
        let non_wrapping_sum = (b.get().0) as i16 - (a.get().0) as i16;

        // overflow flag is set when signed subtraction overflows
        // that happens when inverse of msb of a and msb of b are diffrent
        // but msb of result is diffrent than `or` of them
        let mut overflow = false;
        let msb_a = !a.get().0 >> 7;
        let msb_b = b.get().0 >> 7;
        if (msb_a != msb_b) && ((msb_a | msb_b) != diffrence.0 & 1 << 7) {
            overflow = true;
        }

        self.set_flag('c', non_wrapping_sum >= 0);
        self.set_flag('n', diffrence & Wrapping(128) == Wrapping(128));
        self.set_flag('o', overflow);
        self.set_flag('z', diffrence == Wrapping(0u8));

        diffrence
    }
    pub fn sbc(&mut self, a: &Register, b: &Register) -> Wrapping<u8> {
        let diffrence = b.get() - a.get() - Wrapping(!self.get_flag('c') as u8);
        let non_wrapping_sum = (b.get().0) as i8 - (a.get().0) as i8 - !self.get_flag('c') as i8;

        // overflow flag is set when signed subtraction overflows
        // that happens when inverse of msb of a and msb of b are diffrent
        // but msb of result is diffrent than `or` of them
        let mut overflow = false;
        let msb_a = !(a.get().0 - !self.get_flag('c') as u8) & 1 << 7;
        let msb_b = b.get().0 & 1 << 7;
        if (msb_a != msb_b) && ((msb_a | msb_b) != diffrence.0 & 1 << 7) {
            overflow = true;
        }

        self.set_flag('c', non_wrapping_sum >= 0);
        self.set_flag('n', diffrence & Wrapping(128) == Wrapping(128));
        self.set_flag('o', overflow);
        self.set_flag('z', diffrence == Wrapping(0u8));

        diffrence
    }
    pub fn cmp(&mut self, a: &Register, b: &Register) {
        let diffrence = b.get() - a.get();
        let non_wrapping_sum = (b.get().0) as i8 - (a.get().0) as i8;

        // overflow flag is set when signed subtraction overflows
        // that happens when inverse of msb of a and msb of b are diffrent
        // but msb of result is diffrent than `or` of them
        let mut overflow = false;
        let msb_a = !a.get().0 & 1 << 7;
        let msb_b = b.get().0 & 1 << 7;
        if (msb_a != msb_b) && ((msb_a | msb_b) != diffrence.0 & 1 << 7) {
            overflow = true;
        }

        self.set_flag('c', non_wrapping_sum < 0);
        self.set_flag('n', diffrence & Wrapping(128) == Wrapping(128));
        self.set_flag('o', overflow);
        self.set_flag('z', diffrence == Wrapping(0u8));
    }
    pub fn inc(&mut self, b: &Register) -> Wrapping<u8> {
        let sum = b.get() + Wrapping(1);
        let non_wrapping_sum = (b.get().0) as u16 + 1;

        // overflow flag is set when signed addition overflows
        // that happens when msb of a and msb of b are the same
        // but msb of result is diffrent
        let mut overflow = false;
        let msb_b = (b.get().0 & 1 << 7) as u16;
        let msb_b_post_inc = non_wrapping_sum & 1 << 7;
        if msb_b != msb_b_post_inc {
            overflow = true;
        }

        self.set_flag('c', non_wrapping_sum > 255);
        self.set_flag('n', sum & Wrapping(128) == Wrapping(128));
        self.set_flag('o', overflow);
        self.set_flag('z', sum == Wrapping(0u8));

        sum
    }
    pub fn dec(&mut self, b: &Register) -> Wrapping<u8> {
        let diffrence = b.get() - Wrapping(1);
        let non_wrapping_sum = (b.get().0) as i8 - 1;

        // overflow flag is set when signed addition overflows
        // that happens when msb of a and msb of b are the same
        // but msb of result is diffrent
        let mut overflow = false;
        let msb_b = (b.get().0 & 1 << 7) as i8;
        let msb_b_post_dec = non_wrapping_sum & 1 << 7;
        if msb_b != msb_b_post_dec {
            overflow = true;
        }

        self.set_flag('c', non_wrapping_sum >= 0);
        self.set_flag('n', diffrence & Wrapping(128) == Wrapping(128));
        self.set_flag('o', overflow);
        self.set_flag('z', diffrence == Wrapping(0u8));

        diffrence
    }
    pub fn shl(&mut self, b: &Register) -> Wrapping<u8> {
        let sum: u8 = b.get().0 << 1;

        // overflow flag is set when signed addition overflows
        // that happens when msb of a and msb of b are the same
        // but msb of result is diffrent
        let mut overflow = false;
        let msb_b = (b.get().0 & 1 << 7) as u8;
        let msb_b_post_shl = sum & 1 << 7;
        if msb_b != msb_b_post_shl {
            overflow = true;
        }

        self.set_flag('c', sum < b.get().0);
        self.set_flag('n', (sum & 128) == 128);
        self.set_flag('o', overflow);
        self.set_flag('z', sum == 0);

        Wrapping(sum)
    }
}

// #[cfg(test)]
// impl Arbitrary for Alu {
//     fn arbitrary<G: Gen>(g: &mut G) -> Alu {
//         Alu { flags: [false; 4] }.set()
//     }
// }
