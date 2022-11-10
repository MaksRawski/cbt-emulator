//! 8 Bit Register.
use crate::js::update_dom_number;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Register {
    pub data: u8,
    name: String,
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Register {
    pub fn new(wasm_name: &str) -> Register {
        update_dom_number(wasm_name, 0, 8);
        Register {
            data: 0,
            name: wasm_name.to_string(),
        }
    }
    pub fn o(&self) -> u8 {
        self.data
    }
    /// when setter is None error will be displayed in console.error
    pub fn i(&mut self, v: u8) {
        self.data = v;
        update_dom_number(&self.name, v.into(), 8);
    }
}
