use js_value::map_rust_value;

use wasm_bindgen::prelude::*;

mod closure;
mod constants;
mod environment;
mod evaluator;
mod evaluator_value;
mod js_value;

#[allow(unused_variables)]
#[wasm_bindgen]
pub fn evaluate(ast: String) -> Result<JsValue, JsError> {
    let ast = lib_ir::serialize(ast).map_err(JsError::from)?;

    let eval_result = evaluator::begin_eval(ast).map_err(|e| JsError::new(&e.as_str()))?;

    let js_value = map_rust_value(eval_result);
    Ok(js_value)
}

#[cfg(test)]
mod tests {
    use crate::{constants::JS_UNDEFINED, evaluator, evaluator_value::EvaluatorValue};
    use lib_ir::ast::{literal::JsNumber, literal_value::LiteralValue};
    use wasm_bindgen::prelude::*;

    #[test]
    pub fn add() {
        let ast = r#"
		{"type":"BlockStatement","start":0,"end":4,"body":[{"type":"ExpressionStatement","start":0,"end":4,"expression":{"type":"BinaryExpression","start":0,"end":3,"left":{"type":"Literal","start":0,"end":1,"value":1,"raw":"1"},"operator":"+","right":{"type":"Literal","start":2,"end":3,"value":1,"raw":"1"}}}],"sourceType":"script"}
		"#;
        let ast = lib_ir::serialize(ast.to_string()).expect("Unable to deserialize ast");

        let eval_result = evaluator::begin_eval(ast).map_err(|e| JsError::new(&e.as_str()));

        if let Ok(eval_result) = eval_result {
            if let EvaluatorValue::Literal(eval_result) = eval_result {
                if let LiteralValue::Number(n) = eval_result.value {
                    if let JsNumber::Number(n) = n {
                        assert_eq!(2.0, n);
                        return;
                    }
                }
            }
        }
        unreachable!()
    }

    #[test]
    pub fn variables() {
        let ast = r#"
		{"type":"BlockStatement","start":0,"end":15,"body":[{"type":"VariableDeclaration","start":0,"end":12,"declarations":[{"type":"VariableDeclarator","start":6,"end":11,"id":{"type":"Identifier","start":6,"end":7,"name":"x"},"init":{"type":"Literal","start":10,"end":11,"value":1,"raw":"1"}}],"kind":"const"},{"type":"ExpressionStatement","start":13,"end":15,"expression":{"type":"Identifier","start":13,"end":14,"name":"x"}}],"sourceType":"script"}
		"#;
        let ast = lib_ir::serialize(ast.to_string()).expect("Unable to deserialize ast");

        let eval_result = evaluator::begin_eval(ast).map_err(|e| JsError::new(&e.as_str()));

        if let Ok(eval_result) = eval_result {
            if let EvaluatorValue::Literal(eval_result) = eval_result {
                if let LiteralValue::Number(n) = eval_result.value {
                    if let JsNumber::Number(n) = n {
                        assert_eq!(1.0, n);
                        return;
                    }
                }
            }
        }
        unreachable!()
    }

    #[test]
    pub fn scope() {
        let ast = r#"
		{"type":"BlockStatement","start":0,"end":25,"body":[{"type":"VariableDeclaration","start":0,"end":10,"declarations":[{"type":"VariableDeclarator","start":4,"end":9,"id":{"type":"Identifier","start":4,"end":5,"name":"x"},"init":{"type":"Literal","start":8,"end":9,"value":1,"raw":"1"}}],"kind":"let"},{"type":"BlockStatement","start":11,"end":22,"body":[{"type":"ExpressionStatement","start":14,"end":20,"expression":{"type":"AssignmentExpression","start":14,"end":19,"operator":"=","left":{"type":"Identifier","start":14,"end":15,"name":"x"},"right":{"type":"Literal","start":18,"end":19,"value":2,"raw":"2"}}}]},{"type":"ExpressionStatement","start":23,"end":25,"expression":{"type":"Identifier","start":23,"end":24,"name":"x"}}],"sourceType":"script"}
		"#;
        let ast = lib_ir::serialize(ast.to_string()).expect("Unable to deserialize ast");

        let eval_result = evaluator::begin_eval(ast).expect("Unable to eval");

        if let EvaluatorValue::Literal(eval_result) = eval_result {
            if let LiteralValue::Number(n) = eval_result.value {
                if let JsNumber::Number(n) = n {
                    assert_eq!(2.0, n);
                    return;
                }
            }
        }
        unreachable!()
    }

    #[test]
    pub fn function_call() {
        let ast = r#"
		{"type":"BlockStatement","start":0,"end":37,"body":[{"type":"FunctionDeclaration","start":0,"end":29,"id":{"type":"Identifier","start":9,"end":12,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":15,"end":29,"body":[{"type":"ReturnStatement","start":18,"end":27,"argument":{"type":"Literal","start":25,"end":26,"value":1,"raw":"1"}}]}},{"type":"ExpressionStatement","start":31,"end":37,"expression":{"type":"CallExpression","start":31,"end":36,"callee":{"type":"Identifier","start":31,"end":34,"name":"foo"},"arguments":[],"optional":false}}],"sourceType":"script"}
		"#;
        let ast = lib_ir::serialize(ast.to_string()).expect("Unable to deserialize ast");

        let eval_result = evaluator::begin_eval(ast).expect("Unable to eval");

        if let EvaluatorValue::Literal(eval_result) = eval_result {
            if let LiteralValue::Number(n) = eval_result.value {
                if let JsNumber::Number(n) = n {
                    assert_eq!(1.0, n);
                    return;
                }
            }
        }
        unreachable!()
    }

    #[test]
    pub fn arrow_fn() {
        let ast = r#"
		{"type":"BlockStatement","start":0,"end":109,"body":[{"type":"ExpressionStatement","start":100,"end":108,"expression":{"type":"ArrowFunctionExpression","start":100,"end":107,"id":null,"expression":true,"generator":false,"async":false,"params":[],"body":{"type":"Literal","start":106,"end":107,"value":1,"raw":"1"}}}],"sourceType":"script"}
		"#;
        let ast = lib_ir::serialize(ast.to_string()).expect("Unable to deserialize ast");

        let eval_result = evaluator::begin_eval(ast).expect("Unable to eval");

        if let EvaluatorValue::Literal(eval_result) = eval_result {
            assert_eq!(eval_result.value, JS_UNDEFINED);
        }
    }

    #[test]
    pub fn object_lookup() {
        let ast = r#"
		{"type":"BlockStatement","start":0,"end":26,"body":[{"type":"VariableDeclaration","start":0,"end":21,"declarations":[{"type":"VariableDeclarator","start":4,"end":21,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"ObjectExpression","start":8,"end":21,"properties":[{"type":"Property","start":14,"end":18,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":14,"end":15,"name":"b"},"value":{"type":"Literal","start":17,"end":18,"value":1,"raw":"1"},"kind":"init"}]}}],"kind":"let"},{"type":"ExpressionStatement","start":22,"end":26,"expression":{"type":"MemberExpression","start":22,"end":25,"object":{"type":"Identifier","start":22,"end":23,"name":"a"},"property":{"type":"Identifier","start":24,"end":25,"name":"b"},"computed":false,"optional":false}}],"sourceType":"script"}
		"#;
        let ast = lib_ir::serialize(ast.to_string()).expect("Unable to deserialize ast");

        let eval_result = evaluator::begin_eval(ast).expect("Unable to eval");

        if let EvaluatorValue::Literal(eval_result) = eval_result {
            assert_eq!(eval_result.value, LiteralValue::from(1.0));
        }
    }

    #[test]
    pub fn conditional() {
        let ast = r#"
		{"type":"BlockStatement","start":0,"end":39,"body":[{"type":"VariableDeclaration","start":0,"end":10,"declarations":[{"type":"VariableDeclarator","start":4,"end":9,"id":{"type":"Identifier","start":4,"end":5,"name":"x"},"init":{"type":"Literal","start":8,"end":9,"value":1,"raw":"1"}}],"kind":"let"},{"type":"IfStatement","start":12,"end":35,"test":{"type":"BinaryExpression","start":16,"end":22,"left":{"type":"Identifier","start":16,"end":17,"name":"x"},"operator":"==","right":{"type":"Literal","start":21,"end":22,"value":1,"raw":"1"}},"consequent":{"type":"BlockStatement","start":24,"end":35,"body":[{"type":"ExpressionStatement","start":27,"end":33,"expression":{"type":"AssignmentExpression","start":27,"end":32,"operator":"=","left":{"type":"Identifier","start":27,"end":28,"name":"x"},"right":{"type":"Literal","start":31,"end":32,"value":2,"raw":"2"}}}]},"alternate":null},{"type":"ExpressionStatement","start":37,"end":39,"expression":{"type":"Identifier","start":37,"end":38,"name":"x"}}],"sourceType":"script"}
		"#;
        let ast = lib_ir::serialize(ast.to_string()).expect("Unable to deserialize ast");

        let eval_result = evaluator::begin_eval(ast).expect("Unable to eval");

        if let EvaluatorValue::Literal(eval_result) = eval_result {
            assert_eq!(eval_result.value, LiteralValue::from(2.0));
        }
    }
}
