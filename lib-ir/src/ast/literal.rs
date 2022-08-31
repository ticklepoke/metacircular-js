use serde::Deserialize;

use super::literal_value::{de_from_literal, LiteralValue};

#[derive(Deserialize, Clone)]
pub struct Literal {
    #[serde(deserialize_with = "de_from_literal")]
    pub value: LiteralValue,
}

#[derive(Clone)]
pub enum JsNumber {
    Number(f64),
    Nan,
}

impl PartialEq for JsNumber {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            _ => false, // in JS, NaN === NaN is false
        }
    }
}

impl PartialOrd for JsNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (JsNumber::Number(f1), JsNumber::Number(f2)) => f1.partial_cmp(f2),
            _ => None,
        }
    }
}
