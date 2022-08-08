use lib_ir::ast;

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
pub fn evaluate(_tree: ast::Node) -> Result<ast::Literal, EvaluatorError> {
    // Ok(ast::Literal {
    //     value: ast::LiteralValue::String("hello".into()),
    // })
    Err(EvaluatorError::UknownError)
}
