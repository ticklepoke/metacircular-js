use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use lib_ir::ast::arrow_function::ArrowFunctionExpression;
use lib_ir::ast::coerced_eq::CoercedEq;
use lib_ir::ast::literal::{JsNumber, Literal};
use lib_ir::ast::literal_value::LiteralValue;
use lib_ir::ast::math::{Additive, BitwiseBinary, BitwiseShift, Multiplicative};
use lib_ir::ast::{
    self, AssignmentExpression, AssignmentOperator, BinaryExpression, CallExpression,
    ConditionalExpression, FunctionDeclaration, FunctionExpression, Identifier, IfStatement,
    LogicalExpression, MemberExpression, Node, ObjectExpression, Property, ReturnStatement,
    UnaryExpression, VariableDeclaration, VariableDeclarator,
};
use lib_ir::ast::{BlockStatement, NodeKind};

use crate::closure::Closure;
use crate::constants::{JS_FALSE, JS_NAN, JS_NULL, JS_TRUE, JS_UNDEFINED};
use crate::environment::{Environment, EnvironmentError};
use crate::evaluator_value::EvaluatorValue;

type EvaluatorResult = Result<EvaluatorValue, EvaluatorError>;

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug)]
pub enum EvaluatorError {
    EnvironmentError(EnvironmentError),
    InvalidType(String),
}

impl EvaluatorError {
    pub fn as_str(&self) -> String {
        match self {
            EvaluatorError::EnvironmentError(e) => format!("{:?}", e),
            EvaluatorError::InvalidType(s) => s.to_owned(),
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
        NodeKind::FunctionExpression(f) => eval_function_expression(f, env),
        NodeKind::ArrowFunctionExpression(f) => eval_arrow_function(f, env),
        NodeKind::CallExpression(c) => eval_call_expr(c, env),
        NodeKind::ReturnStatement(r) => eval_return_statement(r, env),
        NodeKind::ObjectExpression(e) => eval_object_expression(e, env),
        NodeKind::MemberExpression(e) => eval_member_expression(e, env),
        NodeKind::IfStatement(e) => eval_if_statement(e, env),
        NodeKind::ConditionalExpression(e) => eval_conditional_expression(e, env),
        _ => unimplemented!("{:?}", tree.kind),
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
        eval_return_statement(return_statement, env)
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
        EvaluatorValue::Object(_) => todo!(),
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
        EvaluatorValue::Object(_) => todo!(),
    };

    let right_value = match right_evaluator_value {
        EvaluatorValue::Literal(l) => l.value,
        EvaluatorValue::Closure(c) => LiteralValue::String(c.to_string()),
        EvaluatorValue::Object(_) => todo!(),
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
fn eval_logical_expression(
    LogicalExpression {
        operator,
        left,
        right,
    }: LogicalExpression,
    env: Env,
) -> EvaluatorResult {
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

fn eval_variable_declaration(
    VariableDeclaration { declarations, kind }: VariableDeclaration,
    env: Env,
) -> EvaluatorResult {
    for d in declarations {
        eval_variable_declarator(d, kind.as_str(), Rc::clone(&env))?;
    }
    Ok(EvaluatorValue::from(JS_NULL))
}

fn eval_variable_declarator(
    VariableDeclarator { id, init }: VariableDeclarator,
    kind: &str,
    env: Env,
) -> EvaluatorResult {
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

fn eval_assignment_expr(
    AssignmentExpression {
        left,
        right,
        operator,
    }: AssignmentExpression,
    env: Env,
) -> EvaluatorResult {
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
fn eval_function_declaration(
    FunctionDeclaration {
        id, params, body, ..
    }: FunctionDeclaration,
    env: Env,
) -> EvaluatorResult {
    let closure = Closure::new(params, body, Some(id.name.to_owned()), Rc::clone(&env));
    // Currently does not hoist
    env.borrow_mut()
        .define(id, EvaluatorValue::from(closure), "let")
        .map_err(EvaluatorError::EnvironmentError)?;
    Ok(EvaluatorValue::from(JS_UNDEFINED))
}

// TODO: no lexical this binding present yet
fn eval_function_expression(
    FunctionExpression {
        id, params, body, ..
    }: FunctionExpression,
    env: Env,
) -> EvaluatorResult {
    let closure = Closure::new(params, body, id.map(|id| id.name), Rc::clone(&env));
    Ok(EvaluatorValue::from(closure))
}

fn eval_arrow_function(
    ArrowFunctionExpression { params, body, .. }: ArrowFunctionExpression,
    env: Env,
) -> EvaluatorResult {
    let normalized_body = match body.kind {
        NodeKind::BlockStatement(b) => b,
        _ => BlockStatement {
            body: vec![Node {
                loc: None,
                kind: NodeKind::ReturnStatement(ReturnStatement {
                    argument: Some(body),
                }),
            }],
        },
    };
    let closure = Closure::new(params, normalized_body, None, Rc::clone(&env));
    Ok(EvaluatorValue::from(closure))
}

// If we call the function with fewer than required args, the rest should default to undefined
// If we call the function with more than the required args, the rest should be ignored
fn eval_call_expr(
    CallExpression { callee, arguments }: CallExpression,
    env: Env,
) -> EvaluatorResult {
    let closure = match callee {
        ast::MemberIdentifier::Identifier(id) => {
            if let EvaluatorValue::Closure(c) = eval_identifier(id, Rc::clone(&env))? {
                c
            } else {
                unreachable!("Trying to call a non function")
            }
        }
        ast::MemberIdentifier::MemberExpression(e) => {
            if let EvaluatorValue::Closure(c) = eval_member_expression(e, Rc::clone(&env))? {
                c
            } else {
                return Err(EvaluatorError::InvalidType(String::from(
                    "Received a non callable value",
                )));
            }
        }
        ast::MemberIdentifier::Expression(_) => todo!(),
        ast::MemberIdentifier::Super(_) => todo!(),
    };

    // eval arguments
    let mut arg_values: Vec<EvaluatorValue> = arguments
        .into_iter()
        .take(closure.parameters.len())
        .map(|arg| evaluate(*arg, Rc::clone(&env)).expect("Unable to evaluate argument"))
        .collect();

    for _ in arg_values.len()..closure.parameters.len() {
        arg_values.push(EvaluatorValue::from(JS_UNDEFINED));
    }

    // extend env with arg values
    let new_env = env.borrow_mut().extend(Rc::clone(&env));
    // set arg values
    closure
        .parameters
        .into_iter()
        .zip(arg_values.into_iter())
        .try_for_each(|(id, value)| {
            if let NodeKind::Identifier(id) = id.kind {
                new_env
                    .borrow_mut()
                    .define(id, value, "let")
                    .map_err(EvaluatorError::EnvironmentError)?;
                Ok(())
            } else {
                unreachable!()
            }
        })?;

    // eval closure body with new env
    eval_block_statement(closure.body, new_env)
}

fn eval_return_statement(r: ReturnStatement, env: Env) -> EvaluatorResult {
    match r.argument {
        None => Ok(EvaluatorValue::from(JS_UNDEFINED)),
        Some(argument) => evaluate(*argument, env),
    }
}

fn eval_object_expression(
    ObjectExpression { properties }: ObjectExpression,
    env: Env,
) -> EvaluatorResult {
    let mut object_frame = HashMap::new();

    properties.into_iter().try_for_each(|p| {
        let Property { key, value, .. } = p;

        let evaluated_value = evaluate(*value, Rc::clone(&env))?;

        let key_string = match key.kind {
            NodeKind::Identifier(id) => id.name,
            NodeKind::Literal(l) => l.value.into(),
            _ => evaluate(*key, Rc::clone(&env))?.into(),
        };
        object_frame.insert(key_string, evaluated_value);
        Ok(())
    })?;

    Ok(EvaluatorValue::Object(Rc::new(RefCell::new(object_frame))))
}

fn eval_member_expression(
    MemberExpression {
        object, property, ..
    }: MemberExpression,
    env: Env,
) -> EvaluatorResult {
    if let EvaluatorValue::Object(obj) = evaluate(*object, Rc::clone(&env))? {
        if let NodeKind::Identifier(id) = property.kind {
            let name = id.name;
            let member = match obj.borrow().get(name.as_str()) {
                Some(value) => match value {
                    EvaluatorValue::Literal(_) | EvaluatorValue::Closure(_) => value.clone(),
                    EvaluatorValue::Object(obj) => EvaluatorValue::Object(Rc::clone(&obj)),
                },
                None => {
                    println!("here");
                    EvaluatorValue::from(JS_UNDEFINED)
                }
            };
            Ok(member)
        } else {
            unreachable!()
        }
    } else {
        return Err(EvaluatorError::InvalidType(String::from(
            "Invalid object lookup",
        )));
    }
}

fn eval_if_statement(
    IfStatement {
        test,
        consequent,
        alternate,
    }: IfStatement,
    env: Env,
) -> EvaluatorResult {
    let test_value: bool = evaluate(*test, Rc::clone(&env))?.into();

    if test_value {
        evaluate(*consequent, Rc::clone(&env))?;
    } else if let Some(alternate) = alternate {
        evaluate(*alternate, Rc::clone(&env))?;
    }

    Ok(EvaluatorValue::from(JS_UNDEFINED))
}

fn eval_conditional_expression(
    ConditionalExpression {
        test,
        alternate,
        consequent,
    }: ConditionalExpression,
    env: Env,
) -> EvaluatorResult {
    let test_value: bool = evaluate(*test, Rc::clone(&env))?.into();

    let body_val = if test_value {
        evaluate(*consequent, Rc::clone(&env))?
    } else {
        evaluate(*alternate, Rc::clone(&env))?
    };

    Ok(body_val)
}
