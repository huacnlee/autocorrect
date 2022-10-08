extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "loadConfig")]
pub fn load(config_str: &str) -> Result<JsValue, wasm_bindgen::JsError> {
    match autocorrect::config::load(config_str) {
        Ok(config) => {
            let val = serde_json::to_value(config).unwrap();
            #[allow(deprecated)]
            let js_value = wasm_bindgen::JsValue::from_serde(&val).unwrap();
            Ok(js_value)
        }
        Err(e) => Err(JsError::new(&format!("{}", e))),
    }
}
