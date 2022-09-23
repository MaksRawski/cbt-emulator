// https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html

use cbt_emulator::cpu::Cpu;
use wasm_bindgen_test::*;

// #[test]
// #[wasm_bindgen_test]
// fn hello_world() {
//     let mut cpu = Cpu::new();
//     // TODO verify this program is correct
//     // tests in cpu.rs pass just fine
//     let hello_world = [
//         0x27, 0xff, 0x3e, 0x01, 0x3e, 0x0f, 0x3e, 0x38, 0x39, 0x00, 0x18, 0x07, 0x00, 0x59, 0xf5,
//         0xf3, 0x2b, 0x00, 0x17, 0x33, 0x2f, 0x00, 0x0d, 0x36, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c,
//         0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x00,
//     ];
//     cpu.load_program(Vec::from(hello_world));
//     while cpu.pc.lo() != 38 {
//         cpu.tick();
//     }
//     dbg!(cpu.lcd.content());
//     assert_eq!(
//         cpu.lcd
//             .content()
//             .unwrap()
//             .iter()
//             .map(|c| *c as char)
//             .collect::<String>(),
//         "Hello world"
//     );
// }

// mov d, 255
// 00 100 111
// 1111 1111
