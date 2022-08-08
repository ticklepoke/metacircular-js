use lib_ir;
use wasm_bindgen::prelude::*;

mod evaluator;

#[wasm_bindgen]
pub fn give(name: &str) -> String {
    name.to_string()
}

#[allow(unused_variables)]
#[wasm_bindgen]
pub fn evaluate(ast: String) -> Option<lib_ir::ast::Literal> {
    let ast = lib_ir::serialize(ast);

	evaluator::evaluate(ast)
    // pass the ast to the evaluator to run
}
