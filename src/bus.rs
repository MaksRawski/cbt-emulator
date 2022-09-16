use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Bus(pub u8);

// impl BUS {
//     pub async fn get(&self) -> u8 {
//         self.0
//     }
//     pub fn put(&mut self, v: u8) {
//         self.0 = v;
//     }
// }

// pub trait Bus {
//     type Output: Future<Output = ()>;

//     fn i(&mut self, bus: &BUS) -> Self::Output;
//     fn o(&self, bus: &mut BUS) -> Self::Output;
// }
