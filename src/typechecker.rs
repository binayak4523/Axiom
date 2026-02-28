use std::collections::HashMap;
use crate::ast::*;
use crate::types::Type;
use crate::diagnostic::Diagnostic;

#[derive(Debug)]
pub struct TypeEnv {
    vars: HashMap<String, Type>,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Type> {
        self.vars.get(name).cloned()
    }

    pub fn set(&mut self, name: String, ty: Type) {
        self.vars.insert(name, ty);
    }
}

pub struct TypeChecker {
    env: TypeEnv,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            env: TypeEnv::new(),
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<Type, Diagnostic> {
        match expr {
            Expr::Number(_) => Ok(Type::Int),
            Expr::Now => Ok(Type::Time),
            Expr::Var(name) => {
                self.env.get(name).ok_or_else(|| {
                    Diagnostic::new(
                        "Unproven Variable",
                        &format!(
                            "The variable '{}' is used here, but no proof exists that it has been defined.",
                            name
                        ),
                    )
                    .with_help(
                        "Define the variable before using it, or pass it as an argument.",
                    )
                })
            }

            Expr::Binary { left, right, .. } => {
                let left_ty = self.check_expr(left)?;
                let right_ty = self.check_expr(right)?;

                if left_ty == Type::Int && right_ty == Type::Int {
                    Ok(Type::Int)
                } else {
                    Err(Diagnostic::new(
                        "Type Mismatch",
                        "Both sides of this operation must have the same numeric type.",
                    ))
                }
            }
        }
    }

    pub fn check(&mut self, stmts: &[Stmt]) -> Result<(), Diagnostic> {
        for stmt in stmts {
            match stmt {
                Stmt::Let { name, value } => {
                    let ty = self.check_expr(value)?;
                    self.env.set(name.clone(), ty);
                }
                Stmt::Expr(expr) => {
                    self.check_expr(expr)?;
                }
            }
        }
        Ok(())
    }
}
