use std::cell::RefCell;
use std::rc::Rc;

use lib_ir::ast::coerced_eq::CoercedEq;
use lib_ir::ast::literal::{JsNumber, Literal};
use lib_ir::ast::literal_value::LiteralValue;
use lib_ir::ast::math::{Additive, BitwiseBinary, BitwiseShift, Multiplicative};
use lib_ir::ast::{
    self, AssignmentExpression, AssignmentOperator, BinaryExpression, FunctionDeclaration,
    Identifier, LogicalExpression, Node, UnaryExpression, VariableDeclaration, VariableDeclarator,
};
use lib_ir::ast::{BlockStatement, NodeKind};

use crate::closure::Closure;
use crate::constants::{JS_FALSE, JS_NAN, JS_NULL, JS_TRUE, JS_UNDEFINED};
use crate::environment::{Environment, EnvironmentError, EvaluatorValue};

type EvaluatorResult = Result<EvaluatorValue, EvaluatorError>;

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug)]
pub enum EvaluatorError {
    EnvironmentError(EnvironmentError),
}

impl EvaluatorError {
    pub fn as_str(&self) -> String {
        match self {
            EvaluatorError::EnvironmentError(e) => format!("{:?}", e),
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
        NodeKind::ExpressionStatement(expr) => evaluate(*expr.expression, env),
        NodeKind::UnaryExpression(expr) => eval_unary_expression(expr, env),
        NodeKind::BinaryExpression(expr) => eval_binary_expression(expr, env),
        NodeKind::LogicalExpression(expr) => eval_logical_expression(expr, env),
        NodeKind::Literal(literal) => Ok(EvaluatorValue::from(literal)),
        NodeKind::VariableDeclaration(decl) => eval_variable_declaration(decl, env),
        NodeKind::Identifier(id) => eval_identifier(id, env),
        NodeKind::AssignmentExpression(expr) => eval_assignment_expr(expr, env),
        NodeKind::FunctionDeclaration(f) => eval_function_declaration(f, env),
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
        return Ok(EvaluatorValue::from(JS_UNDEFINED));
    }
    // TODO: this might be an expensive clone
    let first_seq = seq.first().unwrap().to_owned();
    if seq.len() == 1 {
        return evaluate(first_seq, env);
    }
    if let NodeKind::ReturnStatement(return_statement) = first_seq.kind {
        match return_statement.argument {
            None => Ok(EvaluatorValue::from(JS_UNDEFINED)),
            Some(argument) => evaluate(*argument, env),
        }
    } else {
        // HACK most elegant way to pop the first element
        let mut seq_q = std::collections::VecDeque::from(seq);
        let first = seq_q.pop_front().unwrap();
        evaluate(first, Rc::clone(&env))?;
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

    let Literal { value } = match arg_value {
        EvaluatorValue::Literal(l) => l,
        EvaluatorValue::Closure(_) => unimplemented!(),
    };

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

    Ok(EvaluatorValue::from(evaluated_val))
}

fn eval_binary_expression(expr: BinaryExpression, env: Env) -> EvaluatorResult {
    let BinaryExpression {
        left,
        right,
        operator,
    } = expr;

    let left_evaluator_value = evaluate(*left, Rc::clone(&env))?;
    let right_evaluator_value = evaluate(*right, Rc::clone(&env))?;

    let left_value = match left_evaluator_value {
        EvaluatorValue::Literal(l) => l.value,
        EvaluatorValue::Closure(c) => LiteralValue::String(c.to_string()),
    };

    let right_value = match right_evaluator_value {
        EvaluatorValue::Literal(l) => l.value,
        EvaluatorValue::Closure(c) => LiteralValue::String(c.to_string()),
    };

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

    Ok(EvaluatorValue::from(evaluated_val))
}

// Account for short circuiting behaviour
// https://262.ecma-international.org/5.1/#sec-11.11
fn eval_logical_expression(expr: LogicalExpression, env: Env) -> EvaluatorResult {
    let LogicalExpression {
        operator,
        left,
        right,
    } = expr;

    let left_value = evaluate(*left, Rc::clone(&env))?;

    let left_bool: bool = left_value.into();

    let evaluated_value = match operator {
        ast::LogicalOperator::And => {
            if left_bool {
                evaluate(*right, Rc::clone(&env))?
            } else {
                EvaluatorValue::from(JS_FALSE)
            }
        }
        ast::LogicalOperator::Or => {
            if left_bool {
                EvaluatorValue::from(JS_TRUE)
            } else {
                evaluate(*right, Rc::clone(&env))?
            }
        }
    };
    Ok(evaluated_value)
}

fn eval_variable_declaration(expr: VariableDeclaration, env: Env) -> EvaluatorResult {
    let VariableDeclaration { declarations, kind } = expr;
    println!("declaring");
    for d in declarations {
        eval_variable_declarator(d, kind.as_str(), Rc::clone(&env))?;
    }
    Ok(EvaluatorValue::from(JS_NULL))
}

fn eval_variable_declarator(expr: VariableDeclarator, kind: &str, env: Env) -> EvaluatorResult {
    let VariableDeclarator { id, init } = expr;

    let value = if let Some(init) = init {
        evaluate(*init, Rc::clone(&env))?
    } else {
        EvaluatorValue::from(JS_UNDEFINED)
    };

    env.borrow_mut()
        .define(id, value, kind)
        .map_err(EvaluatorError::EnvironmentError)?;

    Ok(EvaluatorValue::from(JS_NULL))
}

fn eval_identifier(id: Identifier, env: Env) -> EvaluatorResult {
    let evaluator_value = env
        .borrow()
        .lookup(&id)
        .map_or(EvaluatorValue::from(JS_UNDEFINED), |v| v.value);
    Ok(evaluator_value)
}

fn eval_assignment_expr(expr: AssignmentExpression, env: Env) -> EvaluatorResult {
    let AssignmentExpression {
        left,
        right,
        operator,
    } = expr;

    let right_copy = right.clone();
    let right_value = evaluate(*right, Rc::clone(&env))?;

    match operator {
        AssignmentOperator::Eq => {}
        _ => unimplemented!("Only Assignment using = allowed"),
    };

    if let NodeKind::MemberExpression(_) = right_copy.kind {
        todo!("Updating a property of an object")
    } else if let NodeKind::Identifier(id) = left.kind {
        env.borrow_mut()
            .update(id, right_value)
            .map_err(EvaluatorError::EnvironmentError)?;
    } else {
        unreachable!()
    }
    Ok(EvaluatorValue::from(JS_UNDEFINED))
}

// TODO: how to hoist functions?
fn eval_function_declaration(f: FunctionDeclaration, env: Env) -> EvaluatorResult {
    let FunctionDeclaration {
        id, params, body, ..
    } = f;
    let closure = Closure::new(params, body, Rc::clone(&env));
    // Currently does not hoist
    env.borrow_mut()
        .define(id, EvaluatorValue::from(closure), "let")
        .map_err(EvaluatorError::EnvironmentError)?;
    Ok(EvaluatorValue::from(JS_UNDEFINED))
}
