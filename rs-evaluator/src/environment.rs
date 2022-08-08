use std::collections::HashMap;

use lib_ir::ast;

pub enum DeclarationKind {
    Const,
    Let,
    Var,
}

pub struct Variable {
    value: Option<ast::Expression>,
    kind: DeclarationKind,
}

#[derive(Default)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    values: HashMap<ast::Identifier, Variable>,
}

impl Environment {
    pub fn new() -> Self {
        Environment::default()
    }

    pub fn extend(self) -> Self {
        let new_scope = Environment::new();
        new_scope.parent = Some(Box::new(self));
        new_scope
    }

    pub fn lookup(&self, id: &ast::Identifier) -> Option<&Variable> {
        if self.values.contains_key(id) {
            return self.values.get(id);
        }
		let mut curr_env = self.parent;
        while let Some(curr_env) = curr_env {
            if curr_env.values.contains_key(id) {
                return curr_env.values.get(id);
            }
			let curr_env = curr_env.parent;
        }
        None
    }
}
