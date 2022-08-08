use lib_ir::ast;
use lib_ir::ast::{BlockStatement, NodeKind};

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

// TODO change this an eval context struct that collects errors
pub fn evaluate(tree: ast::Node) -> EvaluatorResult {
    match tree.kind {
        NodeKind::Program(_) => unreachable!(),
        NodeKind::BlockStatement(block) => eval_block_statement(block),
        _ => unimplemented!(),
    }
    // Err(EvaluatorError::UknownError)
}

// Create a new env frame, evaluate innerscope
pub fn eval_block_statement(block: BlockStatement) -> EvaluatorResult {

}
