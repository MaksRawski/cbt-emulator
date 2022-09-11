//! 8 Bit Register.
pub use crate::bus::DataBus;
use serde::{Deserialize, Serialize};
use std::num::Wrapping;
use wasm_bindgen::prelude::*;

// #[cfg(test)]
// use quickcheck::{Arbitrary, Gen};

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Register {
    value: Wrapping<u8>,
}

impl Register {
    pub fn new() -> Register {
        Register { value: Wrapping(0) }
    }
}

impl DataBus for Register {
    fn get(&self) -> Wrapping<u8> {
        self.value
    }
    fn set(&mut self, new_value: u8) {
        self.value = Wrapping(new_value);
    }
}

// #[cfg(test)]
// impl Arbitrary for Register {
//     fn arbitrary<G: Gen>(g: &mut G) -> Register {
//         Register { value: Wrapping(g) }
//     }
// }
