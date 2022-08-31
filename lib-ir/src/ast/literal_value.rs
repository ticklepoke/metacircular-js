use super::literal::JsNumber;
use serde::{Deserialize, Deserializer};

use super::{
    coerced_eq::CoercedEq,
    math::{Additive, BitwiseBinary, BitwiseShift, Multiplicative},
};

#[derive(Clone, Debug)]
pub enum LiteralValue {
    String(String),
    Boolean(bool),
    Null,
    Number(JsNumber),
    RegExp,
    Undefined,
}

// This functions maps the estree structure into a more useful LiteralValue
// intermediate repsresentation
// Should we move LiteralValue into a dedicated IR layer and keep this as the AST?
// JSON values can either be strings or numbers, we need to handle each case
pub fn de_from_literal<'de, D>(deserializer: D) -> Result<LiteralValue, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum JsonValue {
        String(Option<String>), // string or null
        F64(f64),
        Bool(bool),
    }

    let res = match JsonValue::deserialize(deserializer)? {
        JsonValue::String(s) => match s {
            Some(s) => LiteralValue::String(s),
            None => LiteralValue::Null,
        },
        JsonValue::F64(f) => LiteralValue::Number(JsNumber::Number(f)),
        JsonValue::Bool(b) => LiteralValue::Boolean(b),
    };
    Ok(res)
}

impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for LiteralValue {
    fn into(self) -> String {
        match self {
            LiteralValue::String(s) => s,
            LiteralValue::Boolean(b) => {
                if b {
                    String::from("true")
                } else {
                    String::from("false")
                }
            }
            LiteralValue::Null => String::from("null"),
            LiteralValue::Number(f) => match f {
                JsNumber::Number(f) => f.to_string(),
                JsNumber::Nan => String::from("NaN"),
            },
            LiteralValue::RegExp => unimplemented!(),
            LiteralValue::Undefined => String::from("undefined"),
        }
    }
}

// https://262.ecma-international.org/5.1/#sec-9.3.1
#[allow(clippy::from_over_into)]
impl Into<JsNumber> for LiteralValue {
    fn into(self) -> JsNumber {
        match self {
            LiteralValue::String(s) => match s.parse::<f64>() {
                Ok(f) => JsNumber::Number(f),
                Err(_) => JsNumber::Nan,
            },
            LiteralValue::Boolean(b) => {
                if b {
                    JsNumber::Number(1.0)
                } else {
                    JsNumber::Number(0.0)
                }
            }
            LiteralValue::Null => JsNumber::Number(0.0),
            LiteralValue::Number(f) => f,
            LiteralValue::RegExp => unimplemented!(),
            LiteralValue::Undefined => JsNumber::Nan,
        }
    }
}

// https://262.ecma-international.org/5.1/#sec-9.2
#[allow(clippy::from_over_into)]
impl Into<bool> for LiteralValue {
    fn into(self) -> bool {
        match self {
            LiteralValue::String(s) => !s.is_empty(),
            LiteralValue::Boolean(b) => b,
            LiteralValue::Number(n) => match n {
                JsNumber::Number(n) => n != 0.0,
                JsNumber::Nan => false,
            },
            LiteralValue::RegExp => unimplemented!(),
            LiteralValue::Null | LiteralValue::Undefined => false,
        }
    }
}

// https://262.ecma-international.org/5.1/#sec-11.8.5
// TODO: into<LiteralValue> if we want to support objects, analagous to ToPrimitive()
impl PartialOrd for LiteralValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (LiteralValue::String(s1), LiteralValue::String(s2)) => s1.partial_cmp(s2),
            _ => {
                // Safety: read only, we can clone
                let n1: JsNumber = self.to_owned().into();
                let n2: JsNumber = other.to_owned().into();
                match (n1, n2) {
                    (JsNumber::Number(n1), JsNumber::Number(n2)) => n1.partial_cmp(&n2),
                    _ => None,
                }
            }
        }
    }
}

macro_rules! binary_op_math {
    ($e1: expr, $e2: expr, $op: tt) => {{
        let n1: JsNumber = $e1.to_owned().into();
        let n2: JsNumber = $e2.to_owned().into();
        match (n1, n2) {
            (JsNumber::Number(n1), JsNumber::Number(n2)) => LiteralValue::from(n1 $op n2),
            _ => LiteralValue::Number(JsNumber::Nan),
        }
    }};
}

// https://262.ecma-international.org/5.1/#sec-11.6
impl Additive for LiteralValue {
    fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (LiteralValue::String(s1), LiteralValue::String(s2)) => {
                LiteralValue::from(format!("{}{}", s1, s2))
            }
            _ => {
                // Safety: read only, we can clone
                let n1: JsNumber = self.to_owned().into();
                let n2: JsNumber = other.to_owned().into();
                match (n1, n2) {
                    (JsNumber::Number(n1), JsNumber::Number(n2)) => LiteralValue::from(n1 + n2),
                    _ => LiteralValue::Number(JsNumber::Nan),
                }
            }
        }
    }

    fn sub(&self, other: &Self) -> Self {
        // Safety: read only, we can clone
        return binary_op_math!(self, other, -);
    }
}

impl Multiplicative for LiteralValue {
    fn mul(&self, other: &Self) -> Self {
        // Safety: read only, we can clone
        return binary_op_math!(self, other, *);
    }

    fn div(&self, other: &Self) -> Self {
        // Safety: read only, we can clone
        return binary_op_math!(self, other, /);
    }

    fn modulo(&self, other: &Self) -> Self {
        // Safety: read only, we can clone
        return binary_op_math!(self, other, %);
    }
}

macro_rules! bitwise_op {
	($e1: expr, $e2: expr, $op: tt) => {{
		let n1: JsNumber = $e1.to_owned().into();
		let n2: JsNumber = $e2.to_owned().into();
		match (n1, n2) {
			(JsNumber::Number(n1), JsNumber::Number(n2)) => {
				let i1 = n1 as i64;
				let i2 = n2 as i64;
				LiteralValue::from((i1 $op i2) as f64)
			}
			_ => LiteralValue::Number(JsNumber::Nan),
		}
	}};
}

// https://262.ecma-international.org/5.1/#sec-11.10
impl BitwiseBinary for LiteralValue {
    fn bitwise_and(&self, other: &Self) -> Self {
        bitwise_op!(self, other, &)
    }

    fn bitwise_or(&self, other: &Self) -> Self {
        bitwise_op!(self, other, |)
    }

    fn bitwise_xor(&self, other: &Self) -> Self {
        bitwise_op!(self, other, ^)
    }
}

// https://262.ecma-international.org/5.1/#sec-11.7
impl BitwiseShift for LiteralValue {
    fn left_shift(&self, other: &Self) -> Self {
        bitwise_op!(self, other, <<)
    }

    fn signed_right_shift(&self, other: &Self) -> Self {
        bitwise_op!(self, other, >>)
    }

    fn unsigned_right_shift(&self, other: &Self) -> Self {
        let n1: JsNumber = self.to_owned().into();
        let n2: JsNumber = other.to_owned().into();
        match (n1, n2) {
            (JsNumber::Number(n1), JsNumber::Number(n2)) => {
                let i1 = if n1 < 0.0 {
                    let b = n1.to_ne_bytes();
                    u64::from_ne_bytes(b) as i64
                } else {
                    n1 as i64
                };
                let i2 = if n2 < 0.0 {
                    let b = n2.to_ne_bytes();
                    u64::from_ne_bytes(b) as i64
                } else {
                    n2 as i64
                };
                LiteralValue::from((i1 >> i2) as f64)
            }
            _ => LiteralValue::Number(JsNumber::Nan),
        }
    }
}

// https://262.ecma-international.org/5.1/#sec-11.9.3
impl CoercedEq for LiteralValue {
    fn coerced_eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Same types
            (LiteralValue::String(s1), LiteralValue::String(s2)) => s1.eq(s2),
            (LiteralValue::Boolean(b1), LiteralValue::Boolean(b2)) => b1.eq(b2),
            (LiteralValue::Number(n1), LiteralValue::Number(n2)) => match (n1, n2) {
                (JsNumber::Number(n1), JsNumber::Number(n2)) => n1.eq(n2),
                _ => false,
            },
            // String and nums
            (LiteralValue::String(_), LiteralValue::Number(n)) => n.eq(&self.to_owned().into()),
            (LiteralValue::Number(n), LiteralValue::String(_)) => n.eq(&other.to_owned().into()),

            (_, LiteralValue::Boolean(_)) | (LiteralValue::Boolean(_), _) => {
                let left_value: JsNumber = self.to_owned().into();
                let right_value: JsNumber = other.to_owned().into();
                left_value.eq(&right_value)
            }

            (_, LiteralValue::RegExp) | (LiteralValue::RegExp, _) => unimplemented!(),
            (LiteralValue::Undefined, _) | (LiteralValue::Null, _) => true,
            _ => false,
        }
    }
}

impl From<f64> for LiteralValue {
    fn from(f: f64) -> Self {
        LiteralValue::Number(JsNumber::Number(f))
    }
}

impl From<String> for LiteralValue {
    fn from(s: String) -> Self {
        LiteralValue::String(s)
    }
}

impl From<&str> for LiteralValue {
    fn from(s: &str) -> Self {
        LiteralValue::String(s.to_string())
    }
}

impl From<bool> for LiteralValue {
    fn from(b: bool) -> Self {
        LiteralValue::Boolean(b)
    }
}
