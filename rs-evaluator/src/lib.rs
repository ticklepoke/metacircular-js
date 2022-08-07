use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn give(name: &str) -> String {
    name.to_string()
}