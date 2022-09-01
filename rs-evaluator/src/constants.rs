use lib_ir::ast::{literal::JsNumber, literal_value::LiteralValue};

pub const JS_NAN: LiteralValue = LiteralValue::Number(JsNumber::Nan);

pub const JS_TRUE: LiteralValue = LiteralValue::Boolean(true);
pub const JS_FALSE: LiteralValue = LiteralValue::Boolean(false);

pub const JS_UNDEFINED: LiteralValue = LiteralValue::Undefined;
pub const JS_NULL: LiteralValue = LiteralValue::Null;
