use serde::Deserialize;

use super::{Identifier, Node, Pattern};

#[derive(Deserialize, Clone, Debug)]
pub struct ArrowFunctionExpression {
    pub id: Option<Identifier>, // always none, but estree spec leaves this null field present
    pub params: Vec<Pattern>,
    pub body: Box<Node>,
    pub expression: bool,
    generator: bool, // false
}
