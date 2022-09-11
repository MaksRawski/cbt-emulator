// TODO:
// 1. create control_word: u32
// 2. change format of these files to be actual bytes
// https://gitlab.com/MaksRawski/cbt/-/blob/master/Microcode/microcode
// 3. When there is a request of instruction
// go to certain byte of a file and set control word
// appropriately.
// microcode.p0 would set first 8 bits of control word
// microcode.p1 would set another 8 bits of control word
// etc.
// 4. create decoder which then based on that control word
// activates appropriate modules by going through CW bit by bit
// (probably).
// 5. use tx,rx as bus

//! Decoding logic.
//! takes value in IR and based on
//! that activates appropriate modules

use crate::cpu::Cpu;
#[warn(unused_imports)]
use serde::{Deserialize, Serialize};
// use std::num::Wrapping;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Microcode {}

impl Microcode {
    pub fn new() -> Self {
        Self {}
    }
    pub fn decode_and_exectue(&mut self, cpu: &mut Cpu) {
        // match type of instruction first
        //     0b00 => {
        //         // mov operations
        //         let src = ir & 0b111;
        //         let dst = (ir >> 3) & 0b111;
        //         match src {
        //             0..=0b11 => {
        //                 // general purpose registers
        //             }
        //             0b100 => {
        //                 // sp is src
        //             }
        //             0b101 => {
        //                 // pc is src
        //             }
        //             0b110 => {
        //                 // send command to lcd
        //             }
        //             0b111 => {
        //                 // mov with immediate opperand or jump
        //             }
        //         }
        //         match dst {}
        //     }
        //     0b01 => {
        //         // load operations
        //         let src = ir & 0b111;
        //         let dst = (ir >> 3) & 0b111;
        //         match src {}
        //         match dst {}
        //     }
        //     0b10 => {
        //         // store operations
        //         let src = ir & 0b111;
        //         let dst = (ir >> 3) & 0b111;
        //     }
        //     0b11 => {
        //         // alu operations
        //     }
        // }
    }
}
