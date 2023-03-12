use js_sys::Date;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/package.js")]
extern "C" {
    fn Dateformat(date: &Date, formatString: &str) -> String;
}

#[wasm_bindgen]
pub fn my_format(date: &Date, format_string: &str) -> String {
    Dateformat(date, format_string)
}