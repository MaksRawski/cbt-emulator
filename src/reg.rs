//! 8 Bit Register.
pub use crate::bus::Bus;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// #[cfg(test)]
// use quickcheck::{Arbitrary, Gen};

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Register(pub u8);

impl Register {
    pub fn new() -> Register {
        Register(0)
    }
    // there will be only one output at a time
    // so they MUST be sync
    pub fn o(&self) -> u8 {
        self.0
    }
    // there can be many inputs (each to a different module)
    // therefore it's safe for them to be async, run concurrently
    pub fn i(&mut self, v: u8) {
        self.0 = v;
    }
}

// #[cfg(test)]
// impl Arbitrary for Register {
//     fn arbitrary<G: Gen>(g: &mut G) -> Register {
//         Register { value: Wrapping(g) }
//     }
// }
