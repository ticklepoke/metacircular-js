use std::{cell::RefCell, collections::HashMap, rc::Rc};

use lib_ir::ast::{self, literal::Literal};

#[derive(Clone)]
pub enum DeclarationKind {
    Const,
    Let,
    Var,
}

impl From<&str> for DeclarationKind {
    fn from(s: &str) -> Self {
        match s {
            "const" => DeclarationKind::Const,
            "let" => DeclarationKind::Let,
            "var" => DeclarationKind::Var,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum EnvironmentError {
    DuplicateDeclaration,
    ReassignmentConst,
    UndefinedVariable,
}

#[derive(Clone)]
pub struct Variable {
    pub value: Literal,
    kind: DeclarationKind,
}

// https://262.ecma-international.org/5.1/#sec-10.2.1
#[derive(Default)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    values: HashMap<ast::Identifier, Variable>,
}

impl Environment {
    pub fn new() -> Self {
        Environment::default()
    }

    // TODO: this leaks implementation details that we need a Rc<RefCell<parent>>
    pub fn extend(&self, parent: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        let mut new_scope = Environment::new();
        new_scope.parent = Some(Rc::clone(&parent));
        Rc::new(RefCell::new(new_scope))
    }

    pub fn define(
        &mut self,
        id: ast::Identifier,
        value: Literal,
        kind: &str,
    ) -> Result<(), EnvironmentError> {
        let k = DeclarationKind::from(kind);
        if self.values.contains_key(&id) {
            return Err(EnvironmentError::DuplicateDeclaration);
        }

        self.values.insert(id, Variable { value, kind: k });
        Ok(())
    }

    pub fn update(&mut self, id: ast::Identifier, value: Literal) -> Result<(), EnvironmentError> {
        if let Some(Variable { kind, .. }) = self.values.get(&id) {
            if let DeclarationKind::Const = kind {
                return Err(EnvironmentError::ReassignmentConst);
            }
            self.values.insert(
                id,
                Variable {
                    value,
                    kind: kind.to_owned(),
                },
            );
            return Ok(());
        };
        // recursively lookup parent frames
        let mut curr: Option<Rc<RefCell<Environment>>>;
        let mut rc: Rc<RefCell<Environment>>;

        curr = match self.parent {
            None => return Err(EnvironmentError::UndefinedVariable),
            Some(ref wrapped_rc) => Some(Rc::clone(wrapped_rc)),
        };
        loop {
            rc = match curr {
                None => return Err(EnvironmentError::UndefinedVariable),
                Some(ref wrapped_rc) => Rc::clone(wrapped_rc),
            };

            let borrowed_env = RefCell::borrow(&rc);
            let maybe_parent = &borrowed_env.parent;
            if let Some(Variable { kind, .. }) = self.values.get(&id) {
                if let DeclarationKind::Const = kind {
                    return Err(EnvironmentError::ReassignmentConst);
                }
                self.values.insert(
                    id,
                    Variable {
                        value,
                        kind: kind.to_owned(),
                    },
                );
                return Ok(());
            } else {
                match maybe_parent {
                    None => return Err(EnvironmentError::UndefinedVariable),
                    Some(ref next_rc) => {
                        curr = Some(Rc::clone(next_rc));
                    }
                };
            }
        }
    }

    pub fn lookup(&self, id: &ast::Identifier) -> Option<Variable> {
        if self.values.contains_key(id) {
            return self.values.get(id).cloned();
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
                return borrowed_env.values.get(id).cloned();
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
