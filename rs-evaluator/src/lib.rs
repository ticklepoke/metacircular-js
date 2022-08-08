use lib_ir;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn give(name: &str) -> String {
    name.to_string()
}

#[allow(unused_variables)]
#[wasm_bindgen]
pub fn evaluate(ast: String) {
    let ast = lib_ir::serialize(ast);

    // pass the ast to the evaluator to run
}
