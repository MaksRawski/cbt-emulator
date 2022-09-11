//! Goes tik-tok.
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Clock {
    pub state: bool,
}

#[wasm_bindgen]
impl Clock {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { state: false }
    }
    /// One clock pulse is two ticks.
    pub fn tick(&mut self) {
        self.state = !self.state;
    }
    #[wasm_bindgen(getter)]
    pub fn get_state(&self) -> bool {
        self.state
    }
}

