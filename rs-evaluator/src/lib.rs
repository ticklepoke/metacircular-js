use js_value::map_rust_value;
use lib_ir;
use wasm_bindgen::prelude::*;

mod constants;
mod environment;
mod evaluator;
mod js_value;

#[allow(unused_variables)]
#[wasm_bindgen]
pub fn evaluate(ast: String) -> Result<JsValue, JsError> {
    let ast = lib_ir::serialize(ast).map_err(|e| JsError::from(e))?;

    let eval_result = evaluator::begin_eval(ast).map_err(|e| JsError::new(e.as_str()))?;

    let js_value = map_rust_value(eval_result);
    Ok(js_value)
}
