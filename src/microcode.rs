// use crate::cpu::Cpu;
// use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

use crate::cw::*;

// #[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Microcode {
    bin: [u32; 65536],
}

impl Microcode {
    /// will panic if microcode in "cbt-ucode/ucode/ucode.bin"
    /// doesn't have 2**16 * 4 bytes
    pub fn load() -> Self {
        let mut f = File::open("cbt-ucode/ucode/ucode.bin").unwrap();
        let mut buffer = [0; 65536 * 4];
        let mut buf = [0u32; 65536];

        f.read_exact(&mut buffer).unwrap();
        let mut i = 0;

        for c in buffer.chunks_exact(4) {
            // convert c to static size array
            let mut bytes = [0u8; 4];
            for j in 0..4 {
                bytes[j] = c[j];
            }
            buf[i] = u32::from_be_bytes(bytes);
            i += 1;
        }
        Self { bin: buf }
    }
    /// flags and utime only care about 4 lowest bits
    pub fn instruction_to_cw(&self, instruction: u8, flags: u8, utime: u8) -> u32 {
        // from microcode generator script:
        //  address is composed as so
        //  FLAGS  MICROTIME INSTRUCTION
        //  xxxx   xxxx      xxxx xxxx
        let flags = ((flags & 0b1111) as usize) << 12;
        let utime = ((utime & 0b1111) as usize) << 8;

        // cw with some bits inverted
        // checkout https://gitlab.com/MaksRawski/cbt/-/wikis/Microcode
        // to see why that is
        let raw_cw = self.bin[(flags | utime | instruction as usize)];

        // since this is an emulator it would be easier
        // if modules where active high
        // therefore everything should be inverted appropriately
        let mask = u32::MAX ^ (MI | PCC | AL0 | AL1 | AL2 | AL3);
        dbg!(raw_cw);
        dbg!(mask);

        raw_cw ^ mask
    }
}

#[cfg(test)]
mod test_microcode {
    use super::*;

    #[test]
    fn test_loading() {
        let _ = Microcode::load();
    }
    #[test]
    fn test_instruction_to_cw() {
        let m = Microcode::load();
        let cw = m.instruction_to_cw(0, 0, 0);
        // at utime = 0 we always fetch the instruction
        // LPO | LAI (Low program counter out, low address in)
        const LPO: u32 = 128;
        const LAI: u32 = 2;
        let expected_cw = LPO | LAI;

        assert_eq!(cw, expected_cw);
    }
}
