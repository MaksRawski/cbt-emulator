// pub use cbt_emulator;
// pub use cbt_emulator::alu::Alu;
// pub use cbt_emulator::bus::DataBus;
// pub use cbt_emulator::lcd::Lcd;
// pub use cbt_emulator::reg::Register;
// // extern crate quickcheck;
// // extern crate quickcheck_macros;

// // use quickcheck::quickcheck;
// // #[macro_use(quickcheck)]
// use std::num::Wrapping;

// mod test_alu {
//     use super::*;

//     // TODO: somehow get rid of this repetetive declaration of variables
//     #[test]
//     fn not() {
//         let mut alu = Alu::new();
//         let mut b = Register::new();

//         b.set(1);
//         assert_eq!(alu.not(&b), Wrapping(254));
//     }

//     #[test]
//     fn nor() {
//         let mut alu = Alu::new();
//         let mut a = Register::new();
//         let mut b = Register::new();

//         a.set(15);
//         b.set(240);
//         assert_eq!(alu.nor(&a, &b), Wrapping(0));
//     }
//     #[test]
//     fn nand() {
//         let mut alu = Alu::new();
//         let mut a = Register::new();
//         let mut b = Register::new();

//         a.set(14);
//         b.set(28);
//         assert_eq!(alu.nand(&a, &b), Wrapping(255 - 12));
//     }
//     #[test]
//     fn xor() {
//         let mut alu = Alu::new();
//         let mut a = Register::new();
//         let mut b = Register::new();

//         a.set(14);
//         b.set(28);
//         assert_eq!(alu.xor(&a, &b), Wrapping(18));
//     }
//     #[test]
//     fn xnor() {
//         let mut alu = Alu::new();
//         let mut a = Register::new();
//         let mut b = Register::new();

//         a.set(14);
//         b.set(28);
//         assert_eq!(alu.xnor(&a, &b), Wrapping(255 - 18));
//     }
//     #[test]
//     fn and() {
//         let mut alu = Alu::new();
//         let mut a = Register::new();
//         let mut b = Register::new();

//         a.set(14);
//         b.set(28);
//         assert_eq!(alu.and(&a, &b), Wrapping(12));
//     }
//     #[test]
//     fn or() {
//         let mut alu = Alu::new();
//         let mut a = Register::new();
//         let mut b = Register::new();

//         a.set(14);
//         b.set(28);
//         assert_eq!(alu.or(&a, &b), Wrapping(30));
//     }

//     #[test]
//     fn add() {
//         let mut alu = Alu::new();
//         let mut a = Register::new();
//         let mut b = Register::new();

//         a.set(127);
//         b.set(127);

//         assert_eq!(alu.add(&a, &b), Wrapping(254));
//         assert_eq!(alu.get_flag('c'), false);
//         assert_eq!(alu.get_flag('n'), true);
//         assert_eq!(alu.get_flag('o'), true);
//         assert_eq!(alu.get_flag('z'), false);
//     }
//     #[test]
//     fn adc() {
//         let mut alu = Alu::new();
//         let mut a = Register::new();
//         let mut b = Register::new();

//         alu.set_flag('c', true);
//         a.set(63);
//         b.set(64);

//         assert_eq!(alu.adc(&a, &b), Wrapping(128));
//         assert_eq!(alu.get_flag('c'), false);
//         assert_eq!(alu.get_flag('n'), true);
//         assert_eq!(alu.get_flag('o'), true);
//         assert_eq!(alu.get_flag('z'), false);
//     }
//     #[test]
//     fn sub() {
//         let mut alu = Alu::new();
//         let mut a = Register::new();
//         let mut b = Register::new();

//         a.set(128);
//         b.set(128);
//         // -128-(-128) = 0

//         assert_eq!(alu.sub(&a, &b), Wrapping(0));
//         assert_eq!(alu.get_flag('c'), true);
//         assert_eq!(alu.get_flag('n'), false);
//         assert_eq!(alu.get_flag('o'), true);
//         assert_eq!(alu.get_flag('z'), true);
//     }
//     #[test]
//     fn sbc() {
//         let mut alu = Alu::new();
//         let mut a = Register::new();
//         let mut b = Register::new();

//         alu.set_flag('c', false);
//         a.set(128);
//         b.set(128);

//         assert_eq!(alu.sbc(&a, &b), Wrapping(255));
//         assert_eq!(alu.get_flag('c'), false);
//         assert_eq!(alu.get_flag('n'), true);
//         assert_eq!(alu.get_flag('o'), false);
//         assert_eq!(alu.get_flag('z'), false);
//     }
//     #[test]
//     fn cmp() {
//         let mut alu = Alu::new();
//         let mut a = Register::new();
//         let mut b = Register::new();

//         a.set(128);
//         b.set(128);

//         alu.cmp(&a, &b);
//         assert_eq!(alu.get_flag('c'), false);
//         assert_eq!(alu.get_flag('n'), false);
//         assert_eq!(alu.get_flag('o'), true);
//         assert_eq!(alu.get_flag('z'), true);
//     }
//     #[test]
//     fn inc() {
//         let mut alu = Alu::new();
//         let mut b = Register::new();

//         b.set(255);

//         assert_eq!(alu.inc(&b), Wrapping(0));
//         assert_eq!(alu.get_flag('c'), true);
//         assert_eq!(alu.get_flag('n'), false);
//         assert_eq!(alu.get_flag('o'), true);
//         assert_eq!(alu.get_flag('z'), true);
//     }
//     #[test]
//     fn dec() {
//         let mut alu = Alu::new();
//         let mut b = Register::new();

//         b.set(0);

//         assert_eq!(alu.dec(&b), Wrapping(255));
//         assert_eq!(alu.get_flag('c'), false);
//         assert_eq!(alu.get_flag('n'), true);
//         assert_eq!(alu.get_flag('o'), true);
//         assert_eq!(alu.get_flag('z'), false);
//     }
//     #[test]
//     fn shl() {
//         let mut alu = Alu::new();
//         let mut b = Register::new();

//         b.set(128);

//         assert_eq!(alu.shl(&b), Wrapping(0));
//         assert_eq!(alu.get_flag('c'), true);
//         assert_eq!(alu.get_flag('n'), false);
//         assert_eq!(alu.get_flag('o'), true);
//         assert_eq!(alu.get_flag('z'), true);
//     }
// }

// mod test_lcd {
//     use super::*;

//     #[test]
//     fn buffer() {
//         let mut lcd = Lcd::new();
//         lcd.txt(67);
//         lcd.txt(66);
//         lcd.txt(84);
//         let mut chars = lcd.display.buffer[0].chars();
//         assert_eq!(chars.next(), Some('C'));
//         assert_eq!(chars.next(), Some('B'));
//         assert_eq!(chars.next(), Some('T'));
//     }
//     #[test]
//     fn cursor_auto_increment() {
//         let mut lcd = Lcd::new();
//         lcd.txt(67);
//         lcd.txt(66);
//         lcd.txt(84);
//         assert_eq!(lcd.cursor.get_row(), 0);
//         assert_eq!(lcd.cursor.get_column(), 3);
//         lcd.cmd(0xc0);
//         assert_eq!(lcd.cursor.get_row(), 1);
//         assert_eq!(lcd.cursor.get_column(), 0);
//     }
//     #[test]
//     fn cursor_manual_increment() {
//         let mut lcd = Lcd::new();
//         for _ in 0..40 {
//             lcd.cmd(6);
//         }
//         assert_eq!(lcd.cursor.get_row(), 1);
//         assert_eq!(lcd.cursor.get_column(), 0);
//         lcd.cmd(4);
//         assert_eq!(lcd.cursor.get_row(), 0);
//         assert_eq!(lcd.cursor.get_column(), 39);
//         lcd.cmd(0xc0);
//         assert_eq!(lcd.cursor.get_row(), 1);
//         assert_eq!(lcd.cursor.get_column(), 0);
//     }
//     #[test]
//     fn shifts() {
//         let mut lcd = Lcd::new();
//         lcd.txt(65);

//         lcd.cmd(0x1c);
//         let mut chars = lcd.display.buffer[0].chars();
//         assert_eq!(chars.next(), Some('\0'));
//         assert_eq!(chars.next(), Some('A'));

//         lcd.cmd(0x18);
//         let mut chars = lcd.display.buffer[0].chars();
//         assert_eq!(chars.next(), Some('A'));
//     }
// }
// mod test_memory {
//     use super::*;

//     #[test]
//     fn get_from_ram() {
//         let mut ram = Ram::new();
//         let mut rom = Rom::new();
//     }
// }
