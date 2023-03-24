//! # Finally you can play around with CBT on your own!
//!
//! This emulator takes real microcode from modified
//! [CBT's microcode generator](https://gitlab.com/MaksRawski/cbt/-/tree/master/Microcode).

pub mod alu;
pub mod clock;
pub mod cpu;
#[allow(dead_code)]
#[macro_use]
pub mod cw;
#[macro_use]
pub mod js;
pub mod interrupts;
pub mod lcd;
pub mod memory;
pub mod microcode;
pub mod pc;
pub mod reg;

use wasm_bindgen::prelude::*;

// wee_alloc is smaller but also slower allocator
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

// #[wasm_bindgen(start)]
// pub fn run() {
//     log(&format!("Hello from {}!", name())); // should output "Hello from Rust!"

//     let r = reg::Register::new();
// }
