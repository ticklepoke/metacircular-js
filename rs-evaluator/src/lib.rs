use lib_ir;
use wasm_bindgen::prelude::*;

mod evaluator;
mod environment;

#[allow(unused_variables)]
#[wasm_bindgen]
pub fn evaluate(ast: String) -> Result<JsValue, JsError> {
    let ast = lib_ir::serialize(ast).map_err(|e| JsError::from(e))?;

    let eval_result = evaluator::begin_eval(ast).map_err(|e| JsError::new(e.as_str()))?;

    // TODO: extract out to separate module
    let js_value = match eval_result.value {
        lib_ir::ast::LiteralValue::String(s) => JsValue::from(s),
        lib_ir::ast::LiteralValue::Boolean(b) => match b {
            true => JsValue::TRUE,
            false => JsValue::FALSE,
        },
        lib_ir::ast::LiteralValue::Null => JsValue::NULL,
        lib_ir::ast::LiteralValue::Number(n) => JsValue::from(n),
        lib_ir::ast::LiteralValue::RegExp => unreachable!(),
        lib_ir::ast::LiteralValue::Undefined => JsValue::UNDEFINED,
    };

    Ok(js_value)
}
