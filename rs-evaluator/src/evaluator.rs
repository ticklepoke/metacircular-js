use std::cell::RefCell;
use std::rc::Rc;

use lib_ir::ast;
use lib_ir::ast::{BlockStatement, NodeKind, Statement};

use crate::environment::Environment;

type EvaluatorResult = Result<ast::Literal, EvaluatorError>;

type Env = Rc<RefCell<Environment>>;

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
    let env = Rc::new(RefCell::new(Environment::new()));
    evaluate(tree, env)
}

// TODO change this an eval context struct that collects errors
pub fn evaluate(tree: ast::Node, env: Env) -> EvaluatorResult {
    match tree.kind {
        NodeKind::Program(_) => unreachable!(),
        NodeKind::BlockStatement(block) => eval_block_statement(block, Rc::clone(&env)),
        _ => unimplemented!(),
    }
}

// Create a new env frame, evaluate innerscope
pub fn eval_block_statement(block: BlockStatement, env: Env) -> EvaluatorResult {
    let body = block.body;
    let inner_env = env.borrow_mut().extend(Rc::clone(&env));
    eval_sequence(body, inner_env)
}

pub fn eval_sequence(_seq: Vec<Statement>, _env: Env) -> EvaluatorResult {
    unimplemented!()
}
