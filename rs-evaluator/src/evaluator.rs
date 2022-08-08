use lib_ir::ast;
use lib_ir::ast::{BlockStatement, NodeKind, Statement};

use crate::environment::Environment;

type EvaluatorResult = Result<ast::Literal, EvaluatorError>;

pub enum EvaluatorError {
    UknownError,
}

impl EvaluatorError {
    pub fn as_str(&self) -> &'static str {
        match self {
            EvaluatorError::UknownError => "Unknown Error",
        }
    }
}

pub fn begin_eval(tree: ast::Node) -> EvaluatorResult {
    let env = Environment::new();
    evaluate(tree, &env)
}

// TODO change this an eval context struct that collects errors
pub fn evaluate(tree: ast::Node, env: &Environment) -> EvaluatorResult {
    match tree.kind {
        NodeKind::Program(_) => unreachable!(),
        NodeKind::BlockStatement(block) => eval_block_statement(block, env),
        _ => unimplemented!(),
    }
    // Err(EvaluatorError::UknownError)
}

// TODO: env sharing might violate ownership rules
// Create a new env frame, evaluate innerscope
pub fn eval_block_statement(block: BlockStatement, env: &Environment) -> EvaluatorResult {
    let body = block.body;
    let inner_env = env.extend();
    eval_sequence(body, &inner_env)
}

pub fn eval_sequence(seq: Vec<Statement>, env: &Environment) -> EvaluatorResult {
    unimplemented!()
}
