pub use crate::bus::DataBus;
use std::num::Wrapping;

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
