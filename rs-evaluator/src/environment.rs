use std::{cell::RefCell, collections::HashMap, rc::Rc};

use lib_ir::ast;

#[derive(Clone)]
pub enum DeclarationKind {
    Const,
    Let,
    Var,
}

#[derive(Clone)]
pub struct Variable {
    value: Option<ast::Expression>,
    kind: DeclarationKind,
}

#[derive(Default)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    values: HashMap<ast::Identifier, Variable>,
}

impl Environment {
    pub fn new() -> Self {
        Environment::default()
    }

    pub fn extend(&self, parent: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        let mut new_scope = Environment::new();
        new_scope.parent = Some(Rc::clone(&parent));
        Rc::new(RefCell::new(new_scope))
    }

    pub fn lookup(&self, id: &ast::Identifier) -> Option<Variable> {
        if self.values.contains_key(id) {
            return self.values.get(id).map(|v| v.clone());
        }
        // recursively lookup parent environments
        let mut curr: Option<Rc<RefCell<Environment>>>;
        let mut rc: Rc<RefCell<Environment>>;

        curr = match self.parent {
            None => return None,
            Some(ref wrapped_rc) => Some(Rc::clone(wrapped_rc)),
        };

        loop {
            rc = match curr {
                None => return None,
                Some(ref wrapped_rc) => Rc::clone(wrapped_rc),
            };

            let borrowed_env = RefCell::borrow(&rc);
            let maybe_parent = &borrowed_env.parent;

            if borrowed_env.values.contains_key(id) {
                return borrowed_env.values.get(id).map(|v| v.clone());
            } else {
                match maybe_parent {
                    None => return None,
                    Some(ref next_rc) => {
                        curr = Some(Rc::clone(next_rc));
                    }
                };
            }
        }
    }
}
