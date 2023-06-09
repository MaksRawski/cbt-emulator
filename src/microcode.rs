use crate::cw::*;

/// Instance of loaded microcode
pub struct Microcode {
    bin: Vec<u32>,
}

impl Microcode {
    /// First decompresses the microcode
    /// then goes through it in chunks of 4 bytes to
    /// convert it into an vector of u32's.
    pub fn load() -> Self {
        let mut buf = Vec::<u8>::with_capacity(262144);
        let mut bin = Vec::<u32>::with_capacity(65536);

        let compressed_ucode = include_bytes!("../ucode/ucode.zstd");
        let mut d = zstd_safe::DCtx::create();

        d.decompress(&mut buf, compressed_ucode)
            .expect("Failed to decompress microcode!");

        for c in buf.chunks_exact(4) {
            // c __doesn't__ have a known size
            // below ensures that it is in fact static array
            // of 4 bytes
            let mut bytes = [0u8; 4];
            for j in 0..4 {
                bytes[j] = c[j];
            }
            bin.push(u32::from_be_bytes(bytes));
        }
        Self { bin }
    }
    /// flags and utime only care about 4 lowest bits
    pub fn instruction_to_cw(&self, instruction: &u8, flags: &u8, utime: &u8) -> u32 {
        // from microcode generator script:
        //  address is composed as so
        //  FLAGS  MICROTIME INSTRUCTION
        //  xxxx   xxxx      xxxx xxxx
        let flags = ((flags & 0b1111) as usize) << 12;
        let utime = ((utime & 0b1111) as usize) << 8;

        // cw with some bits inverted
        // checkout https://gitlab.com/MaksRawski/cbt/-/wikis/Microcode
        // to see why that is
        let raw_cw = self.bin[(flags | utime | *instruction as usize)];

        // since this is an emulator it would be easier
        // if modules where active high
        // therefore everything should be inverted appropriately
        let mask = u32::MAX ^ (MI | PCC | AL0 | AL1 | AL2 | AL3);

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

        // at utime 0..3 we always fetch the instruction
        // 1. LPO | LAI      (low program counter out, low address in)
        // 2. HPO | HAI      (high program counter out, high address in)
        // 3. MO  | II | PCC (Low program counter out, low address in)
        let cw = m.instruction_to_cw(&0, &0, &0);
        assert_eq!(cw, LPO | LAI);

        let cw = m.instruction_to_cw(&0, &0, &1);
        assert_eq!(cw, HPO | HAI);

        let cw = m.instruction_to_cw(&0, &0, &2);
        assert_eq!(cw, MO | II | PCC);
    }
}
