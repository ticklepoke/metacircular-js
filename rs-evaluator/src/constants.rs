use lib_ir::ast::{literal::JsNumber, literal_value::LiteralValue};

pub const JS_NAN: LiteralValue = LiteralValue::Number(JsNumber::Nan);

pub const JS_TRUE: LiteralValue = LiteralValue::Boolean(true);
pub const JS_FALSE: LiteralValue = LiteralValue::Boolean(false);
