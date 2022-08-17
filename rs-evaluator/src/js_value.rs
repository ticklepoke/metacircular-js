use lib_ir::ast::Literal;
use wasm_bindgen::prelude::*;

// TODO: we can evaluate more than just literals, objects would be returned
// as their stringified representation
pub fn map_rust_value(eval_result: Literal) -> JsValue {
    match eval_result.value {
        lib_ir::ast::LiteralValue::String(s) => JsValue::from(s),
        lib_ir::ast::LiteralValue::Boolean(b) => match b {
            true => JsValue::TRUE,
            false => JsValue::FALSE,
        },
        lib_ir::ast::LiteralValue::Null => JsValue::NULL,
        lib_ir::ast::LiteralValue::Number(n) => match n {
            lib_ir::ast::JsNumber::Number(n) => JsValue::from(n),
            lib_ir::ast::JsNumber::Nan => JsValue::from("NaN"), 
        },
        lib_ir::ast::LiteralValue::RegExp => unreachable!(),
        lib_ir::ast::LiteralValue::Undefined => JsValue::UNDEFINED,
    }
}
