use wasm_bindgen::prelude::*;

use crate::alu::Flags;

pub const OFF: char = '○';
pub const ON: char = '●';

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(a: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(a: &str);

}

#[wasm_bindgen]
extern "C" {
    pub fn set_cw(cw: u32);

    pub fn set_flags(flags: JsValue);

}

#[macro_export]
macro_rules! console_log {
     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn update_dom_element(element_id: &str, value: &str) {
    if cfg!(target_family = "wasm") {
        let window = web_sys::window().expect("no global `window` exists");
        let document: web_sys::Document =
            window.document().expect("should have a document in window");
        let _ = document.body().expect("document should have a body");

        let el: web_sys::Element = document
            .get_element_by_id(element_id)
            .expect(&format!("Element with id '{}' not found", element_id));

        el.set_inner_html(value);
    }
}

pub fn update_dom_number(element_id: &str, value: u32, width: u8) {
    if cfg!(target_family = "wasm") {
        update_dom_element(element_id, &to_binary_chars(value.into(), width));

        #[allow(unused_unsafe)]
        unsafe {
            console_log!("Set {}'s value to {}", element_id, value)
        };
    }
}

pub fn update_cw(cw: u32) {
    if cfg!(target_family = "wasm") {
        #[allow(unused_unsafe)]
        unsafe {
            set_cw(cw);
        }
        // for i in 0..32 {
        //     let value = if (cw & 1 << i) > 0 { ON } else { OFF };
        //     update_dom_element(CW_LABELS[i], &value.to_string());
        // }
    }
}

pub fn update_flags(flags: &Flags) -> Result<(), JsValue> {
    #[allow(unused_unsafe)]
    unsafe {
        set_flags(serde_wasm_bindgen::to_value(flags)?);
    }
    Ok(())
}

fn to_binary_chars(num: u32, width: u8) -> String {
    let mut res = String::new();
    for i in (0..width).rev() {
        if (i + 1) % 8 == 0 && i < width - 1 {
            res.push(' ');
        }
        res.push(if num & (1 << i) > 0 { ON } else { OFF });
    }
    res
}

#[test]
fn test_binary_chars() {
    assert_eq!(
        to_binary_chars(0, 8),
        format!("{eight_zeros}", eight_zeros = OFF.to_string().repeat(8))
    );

    assert_eq!(
        to_binary_chars(128, 8),
        format!(
            "{one}{seven_zeros}",
            one = ON,
            seven_zeros = OFF.to_string().repeat(7)
        )
    );

    assert_eq!(
        to_binary_chars(0, 16),
        format!(
            "{eight_zeros} {eight_zeros}",
            eight_zeros = OFF.to_string().repeat(8)
        )
    );
    assert_eq!(
        to_binary_chars(128, 16),
        format!(
            "{eight_zeros} {one}{seven_zeros}",
            eight_zeros = OFF.to_string().repeat(8),
            one = ON,
            seven_zeros = OFF.to_string().repeat(7)
        )
    );
}
