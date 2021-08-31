extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_random() -> String {
    let r = rand::random::<u32>();
    return format!("{:x}", r);
}
