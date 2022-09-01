use lib_ir::ast::{BlockStatement, Expression};

use crate::evaluator::Env;

#[derive(Clone, Debug)]
pub struct Closure {
    pub name: Option<String>,
    pub parameters: Vec<Expression>,
    pub env: Env,
    // single expression arrow functions will be changed into block statements with return statements
    pub body: BlockStatement,
}

impl Closure {
    pub fn new(
        parameters: Vec<Expression>,
        body: BlockStatement,
        name: Option<String>,
        env: Env,
    ) -> Self {
        Closure {
            name,
            parameters,
            env,
            body,
        }
    }
}

impl ToString for Closure {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[allow(clippy::from_over_into)]
impl Into<bool> for Closure {
    fn into(self) -> bool {
        true
    }
}
