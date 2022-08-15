use std::cell::RefCell;
use std::rc::Rc;

use lib_ir::ast::{self, Literal, Node, ReturnStatement};
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

pub fn eval_sequence(seq: Vec<Node>, env: Env) -> EvaluatorResult {
    if seq.is_empty() {
        // Empty block in js should return undefined
        return Ok(Literal {
            value: ast::LiteralValue::Undefined,
        });
    }
    // TODO: this might be an expensive clone
    let first_seq = seq.first().unwrap().to_owned();
    if seq.len() == 1 {
        return evaluate(first_seq, env);
    }
    if let NodeKind::ReturnStatement(return_statement) = first_seq.kind {
        match return_statement.argument {
            None => Ok(Literal {
                value: ast::LiteralValue::Undefined,
            }),
            Some(argument) => evaluate(*argument, env),
        }
    } else {
		// HACK most elegant way to pop the first element
		let mut seq_q = std::collections::VecDeque::from(seq);
		seq_q.pop_front();
		let rest = Vec::from(seq_q);
		eval_sequence(rest, env)
    }
}
