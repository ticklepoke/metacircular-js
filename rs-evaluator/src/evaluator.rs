use std::cell::RefCell;
use std::rc::Rc;

use lib_ir::ast::coerced_eq::CoercedEq;
use lib_ir::ast::literal::{JsNumber, Literal, LiteralValue};
use lib_ir::ast::math::{Additive, BitwiseBinary, BitwiseShift, Multiplicative};
use lib_ir::ast::{self, BinaryExpression, LogicalExpression, Node, UnaryExpression};
use lib_ir::ast::{BlockStatement, NodeKind};

use crate::constants::{JS_FALSE, JS_NAN, JS_TRUE};
use crate::environment::Environment;

type EvaluatorResult = Result<ast::literal::Literal, EvaluatorError>;

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
        NodeKind::BinaryExpression(expr) => eval_binary_expression(expr, env),
        NodeKind::LogicalExpression(expr) => eval_logical_expression(expr, env),
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
            value: ast::literal::LiteralValue::Undefined,
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
                value: ast::literal::LiteralValue::Undefined,
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

// https://262.ecma-international.org/5.1/#sec-11.4
pub fn eval_unary_expression(node: UnaryExpression, env: Env) -> EvaluatorResult {
    let UnaryExpression {
        operator, argument, ..
    } = node;

    let arg_value = evaluate(*argument, env)?;

    let Literal { value } = arg_value;

    let evaluated_val = match value {
        LiteralValue::String(s) => match operator {
            ast::UnaryOperator::Minus | ast::UnaryOperator::Plus => JS_NAN,
            ast::UnaryOperator::Bang => LiteralValue::from(s.is_empty()),
            ast::UnaryOperator::TypeOf => LiteralValue::from("string"),
            ast::UnaryOperator::Void => LiteralValue::from(s),
            ast::UnaryOperator::Delete => JS_TRUE,
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
            ast::UnaryOperator::Delete => JS_TRUE,
        },
        LiteralValue::Null => match operator {
            ast::UnaryOperator::Minus => LiteralValue::from(-0.0),
            ast::UnaryOperator::Plus => LiteralValue::from(0.0),
            ast::UnaryOperator::Bang => JS_TRUE,
            ast::UnaryOperator::TypeOf => LiteralValue::from("object"),
            ast::UnaryOperator::Void => LiteralValue::Undefined,
            ast::UnaryOperator::Delete => JS_TRUE,
        },
        LiteralValue::Number(n) => match n {
            JsNumber::Number(n) => match operator {
                ast::UnaryOperator::Minus => LiteralValue::from(-n),
                ast::UnaryOperator::Plus => LiteralValue::from(n),
                ast::UnaryOperator::Bang => LiteralValue::from(n == 0.0),
                ast::UnaryOperator::TypeOf => LiteralValue::from("number"),
                ast::UnaryOperator::Void => LiteralValue::Undefined,
                ast::UnaryOperator::Delete => JS_TRUE,
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

fn eval_binary_expression(expr: BinaryExpression, env: Env) -> EvaluatorResult {
    let BinaryExpression {
        left,
        right,
        operator,
    } = expr;

    let Literal { value: left_value } = evaluate(*left, Rc::clone(&env))?;
    let Literal { value: right_value } = evaluate(*right, Rc::clone(&env))?;

    let evaluated_val = match operator {
        // https://262.ecma-international.org/5.1/#sec-11.9.3
        ast::BinaryOperator::EqEq => LiteralValue::from(left_value.coerced_eq(&right_value)),
        ast::BinaryOperator::BangEq => LiteralValue::from(left_value.coerced_neq(&right_value)),
        ast::BinaryOperator::EqEqEq => LiteralValue::from(left_value.eq(&right_value)),
        ast::BinaryOperator::BangEqEq => LiteralValue::from(left_value.ne(&right_value)),
        ast::BinaryOperator::Lt => LiteralValue::from(left_value.lt(&right_value)),
        ast::BinaryOperator::Leq => LiteralValue::from(left_value.le(&right_value)),
        ast::BinaryOperator::Gt => LiteralValue::from(left_value.gt(&right_value)),
        ast::BinaryOperator::Geq => LiteralValue::from(left_value.ge(&right_value)),
        ast::BinaryOperator::LtLt => left_value.left_shift(&right_value),
        ast::BinaryOperator::GtGt => left_value.unsigned_right_shift(&right_value),
        ast::BinaryOperator::GtGtGt => left_value.signed_right_shift(&right_value),
        ast::BinaryOperator::Plus => left_value.add(&right_value),
        ast::BinaryOperator::Minus => left_value.sub(&right_value),
        ast::BinaryOperator::Mult => left_value.mul(&right_value),
        ast::BinaryOperator::Div => left_value.div(&right_value),
        ast::BinaryOperator::Mod => left_value.modulo(&right_value),
        ast::BinaryOperator::Pipe => left_value.bitwise_or(&right_value),
        ast::BinaryOperator::Caret => left_value.bitwise_xor(&right_value),
        ast::BinaryOperator::And => left_value.bitwise_and(&right_value),
        ast::BinaryOperator::In => unimplemented!(),
        ast::BinaryOperator::Instanceof => todo!("requires primitive type info"),
    };

    Ok(Literal {
        value: evaluated_val,
    })
}

// Account for short circuiting behaviour
// https://262.ecma-international.org/5.1/#sec-11.11
fn eval_logical_expression(expr: LogicalExpression, env: Env) -> EvaluatorResult {
    let LogicalExpression {
        operator,
        left,
        right,
    } = expr;

    let Literal { value: left_value } = evaluate(*left, Rc::clone(&env))?;

    let left_bool: bool = left_value.into();

    let evaluated_value = match operator {
        ast::LogicalOperator::And => {
            if left_bool {
                let Literal { value: right_value } = evaluate(*right, Rc::clone(&env))?;
                right_value.into()
            } else {
                JS_FALSE
            }
        }
        ast::LogicalOperator::Or => {
            if left_bool {
                JS_TRUE
            } else {
                let Literal { value: right_value } = evaluate(*right, Rc::clone(&env))?;
                right_value.into()
            }
        }
    };
    Ok(Literal {
        value: evaluated_value,
    })
}
