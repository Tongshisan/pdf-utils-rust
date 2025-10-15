use wasm_bindgen::prelude::*;
use std::panic;

mod pdf_utils;
mod image_utils;

pub use pdf_utils::*;
pub use image_utils::*;

// 初始化函数，设置 panic hook 以便在浏览器控制台中看到 Rust 的 panic 信息
#[wasm_bindgen(start)]
pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

