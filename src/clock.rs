use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Clock {
    pub utime: u8,
    pub halted: bool,
}

#[wasm_bindgen]
impl Clock {
    pub fn new() -> Self {
        Self {
            utime: 0,
            halted: false,
        }
    }
    /// utime will overflow if it reaches 16
    /// setting SR bit is preffered way of resetting it
    pub fn tick(&mut self) {
        self.utime = self.utime + 1;
        self.utime &= 0b1111;
    }
    pub fn rst(&mut self) {
        self.utime = 0;
    }
    pub fn hlt(&mut self) {
        self.halted = true;
    }
}
