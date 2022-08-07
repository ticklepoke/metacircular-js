mod ast;

// Creates an estree compliant representation from a serialized ast string
pub fn serialize(ast: String) -> ast::Node {
    let parsed_ast: ast::Node =
        serde_json::from_str(ast.as_str()).expect("Unable to de-serialize AST");
    parsed_ast
}
