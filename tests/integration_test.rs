// https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html

use cbt_emulator::cpu::Cpu;
use wasm_bindgen_test::*;

#[cfg(test)]
mod test_instructions {
    use super::*;
    #[test]
    fn test_mov() {
        let mut cpu = Cpu::new();
        // mov a, 42
        cpu.load_program(vec![0b00_000_111, 42]);

        // mov takes 6 steps
        for _ in 0..6 {
            cpu.tick();
        }
        assert_eq!(cpu.ra.data, 42)
    }
    #[test]
    fn test_jumps() {
        let mut cpu = Cpu::new();

        // jmp test
        // mov a, 2
        // hlt
        //
        // test:
        //  mov a, 42
        //  hlt
        cpu.load_program(vec![
            0b00101111, 0b00000000, 0b00000110, 0b00000111, 0b00000010, 0b00110110, 0b00000111,
            0b00101010, 0b00110110,
        ]);
        for _ in 0..50 {
            cpu.tick();
        }
        assert_eq!(cpu.ra.data, 42);
    }

    #[test]
    fn test_conditional_jumps() {
        let mut cpu = Cpu::new();
        // main:
        //   mov a, 1
        //   dec a
        //   jz zero
        //   mov b, 255
        //   hlt
        // zero:
        //   mov b, 42

        cpu.load_program(vec![
            0x07, 0x01, 0xf8, 0x2b, 0x00, 0x09, 0x0f, 0xff, 0x36, 0x0f, 0x2a,
        ]);
        for _ in 0..40 {
            cpu.tick();
        }
        assert_eq!(cpu.rb.data, 42);
    }

    #[test]
    fn test_loads() {
        let mut cpu = Cpu::new();

        // mov b, 4
        // load a, [cb]
        // hlt
        //
        // #d8 42
        cpu.load_program(vec![
            0b00001111, 0b00000100, 0b01000001, 0b00110110, 0b00101010,
        ]);
        for _ in 0..40 {
            cpu.tick();
        }
        assert_eq!(cpu.ra.data, 42);
    }
}
#[cfg(test)]
mod test_lcd {
    use super::*;

    #[test]
    fn test_lcd_out() {
        let mut cpu = Cpu::new();

        // mov lcd, 42
        // mov lcdc, 0xf // display on, cursor on, blining on
        cpu.load_program(vec![0b00_110_111, 42, 0b00_111_110, 0xf]);

        for _ in 0..7 {
            cpu.tick();
        }
        // display is off so there shouldn't be any value
        assert_eq!(cpu.lcd.content(), None);

        // turning on the display
        for _ in 0..7 {
            cpu.tick();
        }
        assert_eq!(cpu.lcd.content().unwrap()[0], 42);
    }
    #[test]
    fn test_lcd_string() {
        let mut cpu = Cpu::new();

        // mov lcdc, 0x1     ; clear display
        // mov lcdc, 0xF     ; display on, cursor on, blinking on
        // mov lcdc, 0x38    ; function set: 8 bit words, 2 lines
        //
        // mov lcd, 65
        // mov lcd, 66
        // mov lcd, 67
        cpu.load_program(vec![
            0x3e, 0x01, 0x3e, 0x0f, 0x3e, 0x38, 0x37, 0x41, 0x37, 0x42, 0x37, 0x43,
        ]);

        for _ in 0..50 {
            cpu.tick();
        }
        assert_eq!(cpu.lcd.content().unwrap()[0..3], [65, 66, 67]);
    }
}
#[cfg(test)]
mod test_programs {
    use super::*;
    // #[wasm_bindgen_test]
    #[test]
    fn test_hello_world() {
        let mut cpu = Cpu::new();
        //  0:0 |    0 |          ; main:
        //  0:0 |    0 | 27 ff    ; mov SP, 0xFF
        //  2:0 |    2 | 3e 01    ; mov lcdc, 0x1
        //  4:0 |    4 | 3e 0f    ; mov lcdc, 0xF
        //  6:0 |    6 | 3e 38    ; mov lcdc, 0x38
        //  8:0 |    8 | 39 00 18 ; mov cb, [txt]
        //  b:0 |    b | 07 00    ; mov a, 0
        //  d:0 |    d |          ; printStr:
        //  d:0 |    d | 59       ; load d,[cb]
        //  e:0 |    e | f5       ; inc b
        //  f:0 |    f | f3       ; cmp a,d
        // 10:0 |   10 | 2b 00 17 ; jz halt
        // 13:0 |   13 | 33       ; mov lcd,d
        // 14:0 |   14 | 2f 00 0d ; jmp printStr
        // 17:0 |   17 |          ; halt:
        // 17:0 |   17 | 36       ; hlt
        // 18:0 |   18 |          ; txt:
        // 18:0 |   18 | 48 65 6c 6c 6f 2c 20 77 6f 72 6c 64 21 00 ; #d "Hello, world!\0"
        let hello_world = [
            0x3e, 0x01, 0x3e, 0x0f, 0x3e, 0x38, 0x39, 0x00, 0x16, 0x07, 0x00, 0x59, 0xf5, 0xf3,
            0x2b, 0x00, 0x15, 0x33, 0x2f, 0x00, 0x0b, 0x36, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c,
            0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x00,
        ];
        cpu.load_program(Vec::from(hello_world));
        for _ in 0..600 {
            cpu.tick();
        }

        assert_eq!(
            cpu.lcd.string_content(),
            Some(format!("Hello, world!{}", "\0".repeat(32 - 13)))
        );
    }
}
