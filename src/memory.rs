// enum Memory {
//     Ram,
//     Rom,
// }
const RAM_SIZE: usize = 32768;
const ROM_SIZE: usize = 32768;

pub struct Ram {
    pub size: usize,
    pub values: Vec<u8>,
}

pub struct Rom {
    pub size: usize,
    pub values: Vec<u8>,
}
impl Ram {
    pub fn new() -> Self {
        Self {
            size: RAM_SIZE,
            values: Vec::new(),
        }
    }
}
impl Rom {
    pub fn new() -> Self {
        Self {
            size: ROM_SIZE,
            values: Vec::new(),
        }
    }
}
