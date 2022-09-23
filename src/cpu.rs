use std::collections::HashMap;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::alu::ALU;
use crate::bus::Bus;
use crate::clock::Clock;
use crate::cw::*;

use crate::lcd::Lcd;
use crate::memory::Memory;
use crate::microcode::Microcode;
use crate::pc::ProgramCounter;
use crate::reg::Register;

#[wasm_bindgen]
pub struct Cpu {
    react_setters: HashMap<String, JsValue>,
    pub bus: Bus,
    pub clock: Clock,
    pub pc: ProgramCounter,
    #[wasm_bindgen(skip)]
    pub ir: Register,
    mem: Memory,
    ucode: Microcode,

    ra: Register,
    rb: Register,
    rc: Register,
    rd: Register,
    sp: Register,
    alu: ALU,
    pub lcd: Lcd,
}

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Self {
        Self {
            react_setters: HashMap::new(),
            bus: Bus(0),
            clock: Clock::new(),
            ucode: Microcode::load(),
            ir: Register::new("ir"),
            mem: Memory::new(vec![]),
            pc: ProgramCounter::new(),

            ra: Register::new("ra"),
            rb: Register::new("rb"),
            rc: Register::new("rc"),
            rd: Register::new("rd"),
            sp: Register::new("sp"),

            alu: ALU::new(),
            lcd: Lcd::new(),
        }
    }
    pub fn load_program(&mut self, prg: Vec<u8>) {
        self.mem = Memory::new(prg);
    }

    fn cycle(&mut self) {
        let cw = self.ucode.instruction_to_cw(
            &self.ir.data,
            &self.alu.flags.to_byte(),
            &self.clock.utime,
        );

        let bus = match cw {
            cw if (cw & AO > 0) => self.ra.o(),
            cw if (cw & BO > 0) => self.rb.o(),
            cw if (cw & CO > 0) => self.rc.o(),
            cw if (cw & DO > 0) => self.rd.o(),
            cw if (cw & SPO > 0) => self.sp.o(),

            cw if (cw & MO > 0) => self.mem.o(),
            cw if (cw & LPO > 0) => self.pc.lo(),
            cw if (cw & HPO > 0) => self.pc.ho(),
            _ => 0,
        };

        self.bus.0 = bus.clone();

        for i in 0..32 {
            match cw & 1 << i {
                HLT => self.clock.hlt(),

                LAI => self.mem.lai(bus),
                HAI => self.mem.hai(bus),
                MI => self.mem.i(bus),

                II => self.ir.i(bus),
                SR => self.clock.rst(),

                PCC => self.pc.c(),
                LPI => self.pc.li(bus),
                HPI => self.pc.hi(bus),

                AI => self.ra.i(bus),
                BI => self.rb.i(bus),
                CI => self.rc.i(bus),
                DI => self.rd.i(bus),
                SPI => self.sp.i(bus),
                _ => {}
            }
        }
        if cw & LCE > 0 && cw & LCM > 0 {
            self.lcd.cmd(bus);
        } else if (cw & LCE) > 0 {
            self.lcd.txt(bus);
        }
    }
    pub fn tick(&mut self) {
        self.cycle();
        self.clock.tick();
    }
    pub fn view_rom(&self) -> Vec<u8> {
        self.mem.view_rom().to_vec()
    }
    /// setter should be of type `React.Dispatch<React.SetStateAction<any>>`
    pub fn add_setter(&mut self, mod_name: String, setter: JsValue) {
        self.react_setters.insert(mod_name, setter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetching() {
        let mut cpu = Cpu::new();
        cpu.load_program(vec![42]);

        for i in 0..3 {
            let cw = cpu.ucode.instruction_to_cw(
                &cpu.ir.data,
                &cpu.alu.flags.to_byte(),
                &cpu.clock.utime,
            );

            match i {
                0 => assert_eq!(cw, LPO | LAI),
                1 => assert_eq!(cw, HPO | HAI),
                2 => assert_eq!(cw, PCC | MO | II),
                _ => panic!(),
            }

            cpu.tick()
        }
        assert_eq!(cpu.ir.data, 42);
    }
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

    fn fetch(cpu: &mut Cpu) {
        for _ in 0..3 {
            cpu.tick();
        }
        dbg!(cpu.ir.data);
    }
    #[test]
    fn test_lcd_out() {
        let mut cpu = Cpu::new();

        // mov lcd, 42
        // mov lcdc, 0xf // display on, cursor on, blining on
        cpu.load_program(vec![0b00_110_111, 42, 0b00_111_110, 0xf]);

        fetch(&mut cpu);
        // go through instruction itself
        for _ in 0..4 {
            cpu.tick();
        }
        assert_eq!(cpu.lcd.content(), None);

        fetch(&mut cpu);
        // go through instruction itself
        for _ in 0..4 {
            cpu.tick();
        }
        assert_eq!(cpu.lcd.content().unwrap()[0], 42);
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
        dbg!(cpu.pc);
        assert_eq!(cpu.ra.data, 42);
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
    #[test]
    fn test_hello_world() {
        let mut cpu = Cpu::new();
        // outp | addr | data

        //  0:0 |    0 |                            ; main:
        //  0:0 |    0 | 00100111 11111111          ; mov SP, 0xFF
        //  2:0 |    2 | 00111110 00000001          ; mov lcdc, 0x1
        //  4:0 |    4 | 00111110 00001111          ; mov lcdc, 0xF
        //  6:0 |    6 | 00111110 00111000          ; mov lcdc, 0x38
        //  8:0 |    8 | 00111001 00000000 00011000 ; mov cb, [txt]
        //  b:0 |    b | 00000111 00000000          ; mov a, 0
        //  d:0 |    d |                            ; printStr:
        //  d:0 |    d | 01011001                   ; load d,[cb]
        //  e:0 |    e | 11110101                   ; inc b
        //  f:0 |    f | 11110011                   ; cmp a,d
        // 10:0 |   10 | 00101011 00000000 00010111 ; jz halt
        // 13:0 |   13 | 00110011                   ; mov lcd,d
        // 14:0 |   14 | 00101111 00000000 00001101 ; jmp printStr
        // 17:0 |   17 |                            ; halt:
        // 17:0 |   17 | 00110110                   ; hlt
        // 18:0 |   18 |                            ; txt:
        // 18:0 |   18 | 01001000 01100101 01101100 01101100 01101111 00101100 00100000 01110111 01101111 01110010 01101100 01100100 00100001 00000000 ; #d "Hello, world!\0"

        let hello_world = [
            0x27, 0xff, 0x3e, 0x01, 0x3e, 0x0f, 0x3e, 0x38, 0x39, 0x00, 0x18, 0x07, 0x00, 0x59,
            0xf5, 0xf3, 0x2b, 0x00, 0x17, 0x33, 0x2f, 0x00, 0x0d, 0x36, 0x48, 0x65, 0x6c, 0x6c,
            0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x00,
        ];
        cpu.load_program(Vec::from(hello_world));

        // run until halt
        while cpu.ir.data != 0b00_110_110 {
            cpu.tick();
        }

        assert_eq!(
            cpu.lcd
                .content()
                .unwrap()
                .iter()
                .map(|c| *c as char)
                .collect::<String>(),
            "Hello world"
        );
    }
}
