use std::collections::HashMap;

use lib_ir::ast::{literal::Literal, literal_value::LiteralValue};

use crate::closure::Closure;

type JsObject = HashMap<String, EvaluatorValue>;

// An internal representation of js values, including primitives, functions, objects
#[derive(Clone, Debug)]
pub enum EvaluatorValue {
    Literal(Literal),
    Closure(Closure),
    Object(JsObject),
}

impl From<Literal> for EvaluatorValue {
    fn from(l: Literal) -> Self {
        EvaluatorValue::Literal(l)
    }
}

impl From<LiteralValue> for EvaluatorValue {
    fn from(value: LiteralValue) -> Self {
        EvaluatorValue::from(Literal { value })
    }
}

impl From<Closure> for EvaluatorValue {
    fn from(c: Closure) -> Self {
        EvaluatorValue::Closure(c)
    }
}

#[allow(clippy::from_over_into)]
impl Into<bool> for EvaluatorValue {
    fn into(self) -> bool {
        match self {
            EvaluatorValue::Literal(l) => l.value.into(),
            EvaluatorValue::Closure(c) => c.into(),
            EvaluatorValue::Object(_) => true,
        }
    }
}

impl Into<String> for EvaluatorValue {
    fn into(self) -> String {
        match self {
            EvaluatorValue::Literal(l) => l.value.into(),
            EvaluatorValue::Closure(c) => c.to_string(),
            EvaluatorValue::Object(_) => String::from("[object Object]"),
        }
    }
}

impl ToString for EvaluatorValue {
    fn to_string(&self) -> String {
        match self {
            EvaluatorValue::Literal(l) => l.value.to_owned().into(),
            EvaluatorValue::Closure(c) => c.to_string(),
            EvaluatorValue::Object(obj) => {
                let mut s = String::from("{");
                obj.iter().for_each(|(k, v)| {
                    s.push_str(k.as_str());
                    s.push_str(":");
                    s.push_str(v.to_string().as_str());
                    s.push_str(",");
                });
                s.pop();
                s.push_str("}");
                s
            }
        }
    }
}
