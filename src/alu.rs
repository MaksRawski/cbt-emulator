pub use crate::bus::DataBus;
use crate::reg::Register;

use std::collections::HashMap;
use std::num::Wrapping;

pub struct Alu {
    // only contains flags as its internal state
    pub flags: HashMap<char, bool>,
}

impl Alu {
    pub fn new() -> Self {
        let mut flags = HashMap::new();
        flags.insert('c', false);
        flags.insert('n', false);
        flags.insert('o', false);
        flags.insert('z', false);
        Self { flags }
    }

    pub fn set_flag(&mut self, flag: char, value: bool) {
        // 3     2          1        0
        // c     n          o        z
        // carry negtive    overflow zero
        if self.flags.contains_key(&flag) {
            self.flags.insert(flag, value).unwrap();
        } else {
            panic!("{} is not a valid flag.", flag);
        }
    }

    pub fn get_flag(&self, flag: char) -> bool {
        match self.flags.get(&flag) {
            Some(s) => *s,
            None => panic!("{} is not a valid flag.", flag),
        }
    }
}

impl Alu {
    /// argument A needs to get &Register A
    /// argument B is the chosen &Register
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
        let msb_a = a.get().0 & 1 << 7;
        let msb_b = b.get().0 & 1 << 7;
        if (msb_a == msb_b) && ((msb_a & msb_b) != sum.0 & 1 << 7) {
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
        let msb_b = b.get().0 & 1 << 7;
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
        let msb_a = !a.get().0 & 1 << 7;
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
