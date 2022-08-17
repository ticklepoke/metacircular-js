use std::cell::RefCell;
use std::rc::Rc;

use lib_ir::ast::{self, JsNumber, Literal, LiteralValue, Node, UnaryExpression};
use lib_ir::ast::{BlockStatement, NodeKind};

use crate::constants::{JS_FALSE, JS_NAN, JS_TRUE};
use crate::environment::Environment;

type EvaluatorResult = Result<ast::Literal, EvaluatorError>;

type Env = Rc<RefCell<Environment>>;

pub enum EvaluatorError {
    UnknownError,
}

impl EvaluatorError {
    pub fn as_str(&self) -> &'static str {
        match self {
            EvaluatorError::UnknownError => "Unknown Error",
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
        NodeKind::UnaryExpression(expr) => eval_unary_expression(expr, env),
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

pub fn eval_unary_expression(node: UnaryExpression, env: Env) -> EvaluatorResult {
    let UnaryExpression {
        operator, argument, ..
    } = node;

    let arg_value = evaluate(*argument, env)?;

    let Literal { value } = arg_value;

    let evaluated_val = match value {
        LiteralValue::String(s) => match operator {
            ast::UnaryOperator::Minus | ast::UnaryOperator::Plus => JS_NAN,
            ast::UnaryOperator::Bang => LiteralValue::from(s.len() == 0),
            ast::UnaryOperator::TypeOf => LiteralValue::from("string"),
            ast::UnaryOperator::Void => LiteralValue::from(s),
            ast::UnaryOperator::Delete => unimplemented!(), // TODO: This actually depends on whether we want the evaluator to be lazy or eager. Lazy eval will return false while eager eval will return true
        },
        LiteralValue::Boolean(b) => match operator {
            ast::UnaryOperator::Minus => match b {
                true => LiteralValue::from(-1.0),
                false => LiteralValue::from(-0.0),
            },
            ast::UnaryOperator::Plus => match b {
                true => LiteralValue::from(1.0),
                false => LiteralValue::from(0.0),
            },
            ast::UnaryOperator::Bang => LiteralValue::from(!b),
            ast::UnaryOperator::TypeOf => LiteralValue::from("boolean"),
            ast::UnaryOperator::Void => LiteralValue::Undefined,
            ast::UnaryOperator::Delete => unimplemented!(), // TODO: same explanation as deleting strings
        },
        LiteralValue::Null => match operator {
            ast::UnaryOperator::Minus => LiteralValue::from(-0.0),
            ast::UnaryOperator::Plus => LiteralValue::from(0.0),
            ast::UnaryOperator::Bang => JS_TRUE,
            ast::UnaryOperator::TypeOf => LiteralValue::from("object"),
            ast::UnaryOperator::Void => LiteralValue::Undefined,
            ast::UnaryOperator::Delete => unimplemented!(), // TODO: same explanation as deleting strings
        },
        LiteralValue::Number(n) => match n {
            JsNumber::Number(n) => match operator {
                ast::UnaryOperator::Minus => LiteralValue::from(-n),
                ast::UnaryOperator::Plus => LiteralValue::from(n),
                ast::UnaryOperator::Bang => LiteralValue::from(n == 0.0),
                ast::UnaryOperator::TypeOf => LiteralValue::from("number"),
                ast::UnaryOperator::Void => LiteralValue::Undefined,
                ast::UnaryOperator::Delete => todo!(), // TODO: same explanation as deleting strings
            },
            JsNumber::Nan => match operator {
                ast::UnaryOperator::Minus | ast::UnaryOperator::Plus => JS_NAN,
                ast::UnaryOperator::Bang => JS_TRUE,
                ast::UnaryOperator::TypeOf => LiteralValue::from("number"),
                ast::UnaryOperator::Void => LiteralValue::Undefined,
                ast::UnaryOperator::Delete => JS_FALSE,
            },
        },
        LiteralValue::RegExp => match operator {
            ast::UnaryOperator::Minus | ast::UnaryOperator::Plus => JS_NAN,
            ast::UnaryOperator::Bang => JS_FALSE,
            ast::UnaryOperator::TypeOf => LiteralValue::from("object"),
            ast::UnaryOperator::Void => LiteralValue::Undefined,
            ast::UnaryOperator::Delete => todo!(),
        },
        LiteralValue::Undefined => match operator {
            ast::UnaryOperator::Minus | ast::UnaryOperator::Plus => JS_NAN,
            ast::UnaryOperator::Bang => JS_TRUE,
            ast::UnaryOperator::TypeOf => LiteralValue::from("undefined"),
            ast::UnaryOperator::Void => LiteralValue::Undefined,
            ast::UnaryOperator::Delete => JS_FALSE,
        },
    };

    Ok(Literal {
        value: evaluated_val,
    })
}
