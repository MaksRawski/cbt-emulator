//! Communication layer between modules.
//!
//! It includes two seperate buses for data and addresses
//! in comparison to real CBT which has to share single bus for both cases.
use std::num::Wrapping;
pub use wasm_bindgen::prelude::*;

/// 8 Bit data bus
pub trait DataBus {
    fn get(&self) -> Wrapping<u8>;
    fn set(&mut self, new_value: u8);
}

/// 16 Bit address bus
pub trait AddressBus {
    fn get(&self) -> Wrapping<u16>;
    fn set(&mut self, new_value: u16);
}
