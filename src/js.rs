// // take useState setter and
use wasm_bindgen::prelude::*;

const OFF: char = '○';
const ON: char = '●';

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(a: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(a: &str);
}

#[macro_export]
macro_rules! console_log {
     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn update_dom_element(element_id: &str, value: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document: web_sys::Document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let el: web_sys::Element = document
        .get_element_by_id(element_id)
        .expect(&format!("Element with id '{}' not found", element_id));

    el.set_inner_html(value);
}

pub fn update_dom_number(element_id: &str, value: u8) {
    update_dom_element(element_id, &to_binary_chars(value.into(), 8));
    unsafe { console_log!("Set {}'s value to {}", element_id, value) };
}

fn to_binary_chars(num: u16, width: u8) -> String {
    let mut res = String::new();
    for i in (0..width).rev() {
        res.push(if num & (1 << i) > 0 { ON } else { OFF });
    }
    res
}
