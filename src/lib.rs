//! # Finally you can play around with CBT on your own!
//!
//! This emulator takes real microcode
//! straight from [CBT's generator](https://gitlab.com/MaksRawski/cbt/-/tree/master/Microcode).
//! and translates control word which that
//! microcode would set into activating appropriate
//! modules with correct actions.

// pub mod alu;
// pub mod bus;
// pub mod clock;
// pub mod cpu;
// pub mod lcd;
// pub mod memory;
// pub mod microcode;
// pub mod pc;
// pub mod reg;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use std::panic;

#[wasm_bindgen]
pub fn setup_logging() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn add(a: u8, b: u8) -> u8 {
    a + b
}
