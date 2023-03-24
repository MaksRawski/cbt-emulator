// https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html

use cbt_emulator::cpu::Cpu;

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
        // outp | addr | data
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
        cpu.load_program(include_bytes!("hello_world.bin").to_vec());

        for _ in 0..600 {
            cpu.tick();
        }

        assert_eq!(
            cpu.lcd.string_content(),
            Some(format!("Hello, world!{}", "\0".repeat(32 - 13)))
        );
    }
    #[test]
    fn test_interrupts() {
        let mut cpu = Cpu::new();

        // outp   | addr | data
        // 0:0    |    0 | 2f 00 1a    ; jmp main
        // 3:0    |    3 |             ; recv_interrupt_msg:
        // 3:0    |    3 | 52 65 63 65 69 76 65 64 20 69 6e 74 65 72 72 75 70 74 73 3a 20 30 00 ; #d "Received interrupts: 0\0"
        // 1a:0   |   1a |             ; main:
        // 1a:0   |   1a | 27 ff       ; mov SP, 0xFF
        // 1c:0   |   1c | 3e 0e       ; mov lcdc,0xe
        // 1e:0   |   1e | 38 80 00    ; mov dc, num_of_interrupts
        // 21:0   |   21 | 87 00       ; store [dc], 0
        // 23:0   |   23 | 38 00 03    ; mov dc, [recv_interrupt_msg]
        // 26:0   |   26 | a5 2f 00 2e ; call printStr
        // 2a:0   |   2a |             ; .loop:
        // 2a:0   |   2a | 00          ; nop
        // 2b:0   |   2b | 2f 00 2a    ; jmp .loop
        // 2e:0   |   2e |             ; printStr:
        // 2e:0   |   2e | a0          ; push a
        // 2f:0   |   2f | 07 00       ; mov a, 0
        // 31:0   |   31 | 48          ; load b,[dc]
        // 32:0   |   32 | f6          ; inc c
        // 33:0   |   33 | f1          ; cmp a, b
        // 34:0   |   34 | 2b 00 3b    ; jz .ret
        // 37:0   |   37 | 31          ; mov lcd, b
        // 38:0   |   38 | 2f 00 2e    ; jmp printStr
        // 3b:0   |   3b |             ; .ret:
        // 3b:0   |   3b | 44          ; pop a
        // 3c:0   |   3c | 6c          ; ret
        // 7000:0 | 7000 |             ; interrupt:
        // 7000:0 | 7000 | 47 00 80    ; load a, [num_of_interrupts]
        // 7003:0 | 7003 | f4          ; inc a
        // 7004:0 | 7004 | b8 80 00    ; store [num_of_interrupts], a
        // 7007:0 | 7007 | 3e 10       ; mov lcdc, 0x10
        // 7009:0 | 7009 | 6c          ; ret
        cpu.load_program(include_bytes!("interrupts.bin").to_vec());

        while cpu.pc.lo() != 0x1a {
            dbg!(cpu.pc.lo());
            cpu.tick();
        }
    }
}
