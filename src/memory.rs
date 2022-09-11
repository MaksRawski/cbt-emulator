//! Includes definitions of ROM and RAM.
//!
//! CBT uses Harvard architecture (kinda)
//! so ROM is the only place from which instructions
//! can be fetched, while RAM can be used by program.
//! Stack is placed at last 255 bytes of RAM.

// #[macro_use]
use serde::{Deserialize, Serialize};

use std::num::Wrapping;
// use wasm_bindgen::prelude::*;

pub const RAM_SIZE: usize = 32768;
pub const ROM_SIZE: usize = 32768;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Memory {
    Ram,
    Rom,
}

impl Memory {
    pub fn get(rom: &Rom, ram: &Ram, address: Wrapping<u16>) -> u8 {
        if address >> 15 == Wrapping(0) {
            rom.get(address.0 as usize)
        } else {
            ram.get(address.0 as usize)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ram {
    pub values: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rom {
    pub values: Vec<u8>,
}

impl Ram {
    pub fn new() -> Self {
        Self {
            values: vec![0; RAM_SIZE],
        }
    }
    pub fn rst(&mut self) {
        self.values = vec![0; RAM_SIZE];
    }
    pub fn get(&self, address: usize) -> u8 {
        self.values[address]
    }
    // pub fn get(&self,) -> JsValue {
    //     JsValue::from_serde(&self.values).unwrap()
    // }
}

impl Rom {
    pub fn new() -> Self {
        Self {
            values: vec![0; ROM_SIZE],
        }
    }
    pub fn get(&self, address: usize) -> u8 {
        self.values[address]
    }
    // pub fn get(&self) -> JsValue {
    //     JsValue::from_serde(&self.values).unwrap()
    // }
}
