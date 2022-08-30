#![allow(unused_variables)]

use serde_json::Error;

pub mod ast;

// Creates an estree compliant representation from a serialized ast string
pub fn serialize(ast: String) -> Result<ast::Node, Error> {
    let parsed_result: Result<ast::Node, _> = serde_json::from_str(ast.as_str());

    parsed_result
}

#[cfg(test)]
mod tests {
    use crate::ast;

    #[test]
    pub fn add() {
        let ast = r#"
		{"type":"BlockStatement","start":0,"end":4,"body":[{"type":"ExpressionStatement","start":0,"end":4,"expression":{"type":"BinaryExpression","start":0,"end":3,"left":{"type":"Literal","start":0,"end":1,"value":1,"raw":"1"},"operator":"+","right":{"type":"Literal","start":2,"end":3,"value":1,"raw":"1"}}}],"sourceType":"script"}
		"#;
        let parsed_ast: ast::Node = serde_json::from_str(ast).expect("Unable to de-serialize AST");
    }

    #[test]
    pub fn object_literal() {
        let ast = r#"
		{"type":"BlockStatement","start":0,"end":34,"body":[{"type":"VariableDeclaration","start":0,"end":21,"declarations":[{"type":"VariableDeclarator","start":4,"end":21,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"ObjectExpression","start":8,"end":21,"properties":[{"type":"Property","start":14,"end":18,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":14,"end":15,"name":"b"},"value":{"type":"Literal","start":17,"end":18,"value":1,"raw":"1"},"kind":"init"}]}}],"kind":"let"},{"type":"ExpressionStatement","start":22,"end":30,"expression":{"type":"AssignmentExpression","start":22,"end":29,"operator":"=","left":{"type":"MemberExpression","start":22,"end":25,"object":{"type":"Identifier","start":22,"end":23,"name":"a"},"property":{"type":"Identifier","start":24,"end":25,"name":"c"},"computed":false,"optional":false},"right":{"type":"Literal","start":28,"end":29,"value":2,"raw":"2"}}},{"type":"ExpressionStatement","start":32,"end":34,"expression":{"type":"Identifier","start":32,"end":33,"name":"a"}}],"sourceType":"script"}
		"#;
        let parsed_ast: ast::Node = serde_json::from_str(ast).expect("Unable to de-serialize AST");
    }
}
