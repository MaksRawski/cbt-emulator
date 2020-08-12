use std::num::Wrapping;

pub trait DataBus {
    fn get(&self) -> Wrapping<u8>;
    fn set(&mut self, new_value: u8);
}

pub trait AddressBus {
    fn get(&self) -> Wrapping<u16>;
    fn set(&mut self, new_value: u16);
}

