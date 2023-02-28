use std::num::Wrapping;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::js::{halt, log, update_dom_number};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Clock {
    pub utime: u8,
    pub halted: bool,
}

#[wasm_bindgen]
impl Clock {
    pub fn new() -> Self {
        update_dom_number("utime", 0, 4);
        Self {
            utime: 0,
            halted: false,
        }
    }
    /// utime will overflow if it reaches 16
    /// setting SR bit is preffered way of resetting it
    pub fn tick(&mut self) {
        if !self.halted {
            self.utime = (Wrapping(self.utime) + Wrapping(1)).0;
            self.utime &= 0b1111;
        }
        update_dom_number("utime", self.utime.into(), 4);
    }
    pub fn rst(&mut self) {
        self.utime = u8::MAX;
        update_dom_number("utime", 0, 4);
    }
    pub fn hlt(&mut self) {
        self.halted = true;
        if cfg!(target_family = "wasm") {
            #[allow(unused_unsafe)]
            unsafe {
                log("should halt now!");
                halt();
            }
        }
    }
}
