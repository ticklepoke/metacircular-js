use lib_ir::ast::{literal::JsNumber, literal_value::LiteralValue};
use wasm_bindgen::prelude::*;

use crate::environment::EvaluatorValue;

// TODO: we can evaluate more than just literals, objects would be returned
// as their stringified representation
pub fn map_rust_value(eval_result: EvaluatorValue) -> JsValue {
    match eval_result {
        EvaluatorValue::Closure(c) => JsValue::from_str(c.to_string().as_str()),
        EvaluatorValue::Literal(l) => match l.value {
            LiteralValue::String(s) => JsValue::from(s),
            LiteralValue::Boolean(b) => match b {
                true => JsValue::TRUE,
                false => JsValue::FALSE,
            },
            LiteralValue::Null => JsValue::NULL,
            LiteralValue::Number(n) => match n {
                JsNumber::Number(n) => JsValue::from_f64(n),
                JsNumber::Nan => JsValue::from("NaN"),
            },
            LiteralValue::RegExp => unreachable!(),
            LiteralValue::Undefined => JsValue::UNDEFINED,
        },
    }
}
