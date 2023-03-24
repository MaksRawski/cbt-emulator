pub struct ProgrammableInterruptController {
    /// mask = true, means it's disabled
    pub mask: bool,
    /// value of the nth-bit from the right represents
    /// whether the device with that id has requested an interrupt
    queue: u8,
}

impl ProgrammableInterruptController {
    pub fn new() -> Self {
        Self {
            mask: true,
            queue: 0,
        }
    }

    pub fn enable(&mut self) {
        self.mask = false;
    }

    pub fn request(&mut self, device_id: u8) {
        self.queue |= device_id;
    }

    /// returns HPC for the jump address for the highest priority device (one with the lowest id)
    /// and marks that device as handled.
    ///
    /// | IO device | jump address |
    /// |-----------+--------------|
    /// | device 0  |       0x7000 |
    /// | device 1  |       0x7200 |
    /// | device 2  |       0x7400 |
    /// | device 3  |       0x7600 |
    /// | device 4  |       0x7800 |
    /// | device 5  |       0x7A00 |
    /// | device 6  |       0x7C00 |
    /// | device 7  |       0x7E00 |
    ///
    /// e.g. for the queue = 0b0100_1010
    /// 1. handle() -> 0x72
    /// 2. handle() -> 0x76
    /// 3. handle() -> 0x7C
    ///
    pub fn handle(&mut self) -> u8 {
        // .trailing_zeros returns u32 for some reason but
        // since the input number is u8 it's safe to unwrap
        // also as it's an assembly instruction in both x86 (bsf) and wasm (ctz) it's incredibly fast
        let device_id: u8 = self.queue.trailing_zeros().try_into().unwrap();

        // mark the device as handled
        self.queue ^= device_id;

        return 0x7 << 4 | self.queue << 1;
    }
}
