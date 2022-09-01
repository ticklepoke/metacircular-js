use js_value::map_rust_value;

use wasm_bindgen::prelude::*;

mod constants;
mod environment;
mod evaluator;
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
    use crate::evaluator;
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
    pub fn variables() {
        let ast = r#"
		{"type":"BlockStatement","start":0,"end":15,"body":[{"type":"VariableDeclaration","start":0,"end":12,"declarations":[{"type":"VariableDeclarator","start":6,"end":11,"id":{"type":"Identifier","start":6,"end":7,"name":"x"},"init":{"type":"Literal","start":10,"end":11,"value":1,"raw":"1"}}],"kind":"const"},{"type":"ExpressionStatement","start":13,"end":15,"expression":{"type":"Identifier","start":13,"end":14,"name":"x"}}],"sourceType":"script"}
		"#;
        let ast = lib_ir::serialize(ast.to_string()).expect("Unable to deserialize ast");

        let eval_result = evaluator::begin_eval(ast).map_err(|e| JsError::new(&e.as_str()));

        if let Ok(eval_result) = eval_result {
            if let LiteralValue::Number(n) = eval_result.value {
                if let JsNumber::Number(n) = n {
                    assert_eq!(1.0, n);
					return;
                }
            }
        } 
		unreachable!()
    }
}
