//! Emulates HD74480 lcd 16x2 variant.

use serde::{Deserialize, Serialize};
pub use wasm_bindgen::prelude::*;

use crate::js::update_lcd;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(a: &str);
// }

// macro_rules! console_log {
//      ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Lcd {
    display: Display,
    cursor: Cursor,
}

#[derive(Serialize, Deserialize)]
pub struct Display {
    pub buffer: Vec<u8>,
    two_line_mode: bool,
    on: bool,
}

#[derive(Serialize, Deserialize)]
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
        self.row as usize * 80 + self.column as usize
    }
    pub fn increment(&mut self) {
        if self.column == 79 {
            if self.row == 0 {
                self.row = 1;
                self.column = 0;
            } else {
                return;
                // self.row = 0;
                // self.column = 0;
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
                self.column = 79;
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
            buffer: vec![0; 80],
            on: false,
            two_line_mode: false,
        }
    }
    pub fn clear(&mut self) {
        self.buffer = vec![0; 80];
    }
    pub fn add(&mut self, cur: &mut Cursor, charcode: u8) {
        self.buffer[cur.location()] = charcode;

        // send this code to display
        // and js will decide which character
        // corresponds to that code
        cur.increment();
    }
    pub fn shift_right(&mut self) {
        let mut new_buffer = vec![0u8; 80];
        new_buffer[0] = 0;
        for i in 1..80 {
            new_buffer[i] = self.buffer[i - 1];
        }
        self.buffer = new_buffer;
    }
    pub fn shift_left(&mut self) {
        let mut new_buffer = vec![0u8; 80];
        for i in 0..79 {
            new_buffer[i] = self.buffer[i + 1];
        }
        new_buffer[79] = 0;
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
        update_lcd(&self).unwrap();
    }
    pub fn cmd(&mut self, cmd: u8) {
        match cmd {
            0x1 => self.clr(),
            0x2 => self.cursor.return_home(),
            0x4 => self.cursor.decrement(),
            0x6 => self.cursor.increment(),
            0x5 => self.display.shift_right(),
            0x7 => self.display.shift_left(),
            0x8 => {
                self.display.off();
                self.cursor.hide();
            }
            0xa => self.display.off(),
            0xc => self.cursor.hide(),
            0xf => {
                self.display.on();
                self.cursor.show();
                self.cursor.blinking = true;
            }
            0x10 => self.cursor.decrement(),
            0x14 => self.cursor.increment(),
            0x18 => self.display.shift_left(),
            0x1c => self.display.shift_right(),
            0x38 => {
                // 8 bit mode and
                self.display.two_line_mode()
            }
            0x80 => self.cursor.return_home(),
            0xc0 => self.cursor.second_row(),
            _ => {}
        }
        update_lcd(&self).unwrap();
    }
}

impl Lcd {
    /// returns vector of 32 bytes (if the display is on)
    pub fn content(&self) -> Option<Vec<u8>> {
        if self.display.on {
            let mut out = Vec::with_capacity(32);
            out.append(&mut self.display.buffer[0..16].to_vec());
            out.append(&mut self.display.buffer[64..80].to_vec());

            return Some(out);
        }
        None
    }
    pub fn string_content(&self) -> Option<String> {
        if let Some(v) = self.content() {
            let s = v.iter().map(|c| *c as char).collect::<String>();
            return Some(s);
        }
        return None;
    }
}

#[cfg(test)]
mod test_lcd {
    use super::*;

    #[test]
    fn buffer() {
        let mut lcd = Lcd::new();
        lcd.txt(67);
        lcd.txt(66);
        lcd.txt(84);
        let chars = lcd.display.buffer;
        assert_eq!(chars[0..=2], [67, 66, 84]);
    }
    #[test]
    fn cursor_auto_increment() {
        let mut lcd = Lcd::new();
        lcd.txt(67);
        lcd.txt(66);
        lcd.txt(84);
        assert_eq!(lcd.cursor.row, 0);
        assert_eq!(lcd.cursor.column, 3);
        lcd.cmd(0xc0);
        assert_eq!(lcd.cursor.row, 1);
        assert_eq!(lcd.cursor.column, 0);
    }
    #[test]
    fn cursor_manual_increment() {
        let mut lcd = Lcd::new();
        for _ in 0..80 {
            lcd.cmd(6);
        }
        assert_eq!(lcd.cursor.row, 1);
        assert_eq!(lcd.cursor.column, 0);
        lcd.cmd(4);
        assert_eq!(lcd.cursor.row, 0);
        assert_eq!(lcd.cursor.column, 79);
        lcd.cmd(0xc0);
        assert_eq!(lcd.cursor.row, 1);
        assert_eq!(lcd.cursor.column, 0);
    }
    #[test]
    fn shifts() {
        let mut lcd = Lcd::new();
        lcd.txt(65);
        lcd.txt(66);
        lcd.txt(67);

        // turn on
        lcd.cmd(0xf);

        // shift right
        lcd.cmd(0x1c);
        assert_eq!(lcd.string_content().unwrap()[0..4], "\0ABC".to_string());

        // shift left
        lcd.cmd(0x18);
        assert_eq!(lcd.string_content().unwrap()[0..3], "ABC".to_string());
    }
    #[test]
    fn test_init() {
        // mov lcdc, 0x1 				; clear display
        // mov lcdc, 0xF 				; display on, cursor on, blinking on
        // mov lcdc, 0x38 				; function set: 8 bit words, 2 lines
    }
}
