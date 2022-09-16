//! Emulates HD74480 lcd 16x2 variant.

pub use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(a: &str);
// }

// macro_rules! console_log {
//      ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Lcd {
    display: Display,
    cursor: Cursor,
}

#[derive(Copy, Clone, Debug)]
pub struct Display {
    pub buffer: [u8; 40],
    two_line_mode: bool,
    on: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct Cursor {
    row: u8,
    column: u8,
    visible: bool,
    blinking: bool,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            row: 0,
            column: 0,
            visible: false,
            blinking: false,
        }
    }
    pub fn return_home(&mut self) {
        self.row = 0;
        self.column = 0;
    }
    pub fn show(&mut self) {
        self.visible = true;
    }
    pub fn hide(&mut self) {
        self.visible = false;
    }
    pub fn location(&self) -> usize {
        self.row as usize * 40 + self.column as usize
    }
    pub fn increment(&mut self) {
        if self.column == 39 {
            if self.row == 0 {
                self.row = 1;
                self.column = 0;
            } else {
                self.row = 0;
                self.column = 0;
                // or go out of buffer
                // or do nothing
            }
        } else {
            self.column += 1;
        }
    }
    pub fn decrement(&mut self) {
        if self.column == 0 {
            if self.row == 1 {
                self.row = 0;
                self.column = 39;
            }
        } else {
            self.column -= 1;
        }
    }
    pub fn second_row(&mut self) {
        self.row = 1;
        self.column = 0;
    }
}

impl Display {
    pub fn new() -> Self {
        Self {
            buffer: [0; 40],
            on: false,
            two_line_mode: false,
        }
    }
    pub fn clear(&mut self) {
        self.buffer = [0; 40];
    }
    pub fn add(&mut self, cur: &mut Cursor, charcode: u8) {
        self.buffer[cur.location()] = charcode;

        // send this code to display
        // and js will decide which character
        // corresponds to that code
        cur.increment();
    }
    pub fn shift_right(&mut self) {
        let mut new_buffer = [0u8; 40];
        new_buffer[0] = 0;
        for i in 1..40 {
            new_buffer[i] = self.buffer[i - 1];
        }
        self.buffer = new_buffer;
    }
    pub fn shift_left(&mut self) {
        let mut new_buffer = [0u8; 40];
        for i in 0..39 {
            new_buffer[i] = self.buffer[i + 1];
        }
        new_buffer[39] = 0;
        self.buffer = new_buffer;
    }

    // keep buffer but turn off screen
    pub fn off(&mut self) {
        self.on = false;
    }

    // display buffer on screen
    pub fn on(&mut self) {
        self.on = true;
    }

    // by default only show 1st line
    // but after enabling it show the
    // 2nd one also
    pub fn two_line_mode(&mut self) {
        self.two_line_mode = true;
    }
}

impl Lcd {
    pub fn new() -> Self {
        let display = Display::new();
        let cursor = Cursor::new();
        Self { display, cursor }
    }
    fn clr(&mut self) {
        self.display.clear();
        self.cursor.return_home();
    }
    pub fn txt(&mut self, chr: u8) {
        self.display.add(&mut self.cursor, chr);
    }
    pub fn cmd(&mut self, cmd: u8) {
        match cmd {
            1 => self.clr(),
            2 => self.cursor.return_home(),
            4 => self.cursor.decrement(),
            6 => self.cursor.increment(),
            5 => self.display.shift_right(),
            7 => self.display.shift_left(),
            8 => {
                self.display.off();
                self.cursor.hide();
            }
            10 => self.display.off(),
            0xc => self.cursor.hide(),
            0xe | 0xf => {
                self.display.on();
                self.cursor.blinking = true;
            }
            0x10 => self.cursor.decrement(),
            0x14 => self.cursor.increment(),
            0x18 => self.display.shift_left(),
            0x1c => self.display.shift_right(),
            0x38 => self.display.two_line_mode(),
            0x80 => self.cursor.return_home(),
            0xc0 => self.cursor.second_row(),
            _ => {}
        }
    }
}

#[wasm_bindgen]
impl Lcd {
    pub fn content(&self) -> Vec<u8> {
        Vec::from(self.display.buffer)
    }
}
