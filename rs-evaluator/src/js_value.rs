use lib_ir::ast::literal::{JsNumber, Literal, LiteralValue};
use wasm_bindgen::prelude::*;

// TODO: we can evaluate more than just literals, objects would be returned
// as their stringified representation
pub fn map_rust_value(eval_result: Literal) -> JsValue {
    match eval_result.value {
        LiteralValue::String(s) => JsValue::from(s),
        LiteralValue::Boolean(b) => match b {
            true => JsValue::TRUE,
            false => JsValue::FALSE,
        },
        LiteralValue::Null => JsValue::NULL,
        LiteralValue::Number(n) => match n {
            JsNumber::Number(n) => JsValue::from(n),
            JsNumber::Nan => JsValue::from("NaN"),
        },
        LiteralValue::RegExp => unreachable!(),
        LiteralValue::Undefined => JsValue::UNDEFINED,
    }
}
