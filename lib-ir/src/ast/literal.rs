use serde::Deserialize;

use super::math::{Additive, BitwiseBinary, BitwiseShift, Multiplicative};

#[derive(Deserialize, Clone)]
pub struct Literal {
    pub value: LiteralValue,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum LiteralValue {
    String(String),
    Boolean(bool),
    Null,
    #[serde(skip)] // TODO: this representation does not correspond to js
    Number(JsNumber),
    RegExp,
    Undefined,
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
