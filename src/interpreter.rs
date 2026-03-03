use std::collections::HashMap;
use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Time(i64),
}

#[derive(Debug)]
pub struct Env {
    scopes: Vec<HashMap<String, Value>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn insert(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }
}

pub struct Interpreter {
    env: Env,
    time: i64,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Env::new(),
            time: 0,
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Int(*n),

            Expr::Var(name) => self.env
                .get(name)
                .unwrap_or_else(|| panic!("Undefined variable '{}'", name)),

            Expr::Binary { left, op, right } => {
                let l = self.eval_expr(left);
                let r = self.eval_expr(right);

                match (l, r, op) {
                    (Value::Int(a), Value::Int(b), BinOp::Add) => Value::Int(a + b),
                    (Value::Int(a), Value::Int(b), BinOp::Sub) => Value::Int(a - b),
                    (Value::Int(a), Value::Int(b), BinOp::Mul) => Value::Int(a * b),
                    (Value::Int(a), Value::Int(b), BinOp::Div) => {
                        if b == 0 {
                            panic!("Division by zero");
                        }
                        Value::Int(a / b)
                    }
                    _ => panic!("Type error: cannot apply binary operation to these values"),
                }
            }
            Expr::Now => {
                let t = self.time;
                self.time += 1; // advance time deterministically
                Value::Time(t)
            }

        }
    }

    pub fn execute(&mut self, stmts: &[Stmt]) -> Option<Value> {
        let mut last = None;

        for stmt in stmts {
            match stmt {
                Stmt::Let { name, value } => {
                    let val = self.eval_expr(value);
                    self.env.insert(name.clone(), val);
                }
                Stmt::Expr(expr) => {
                    last = Some(self.eval_expr(expr));
                }
                Stmt::Block(block_stmts) => {
                    self.env.enter_scope();

                    let mut block_last = None;
                    for block_stmt in block_stmts {
                        match block_stmt {
                            Stmt::Let { name, value } => {
                                let val = self.eval_expr(value);
                                self.env.insert(name.clone(), val);
                            }
                            Stmt::Expr(expr) => {
                                block_last = Some(self.eval_expr(expr));
                            }
                            Stmt::Block(_) => {
                                // Handle nested blocks recursively
                                block_last = self.execute(std::slice::from_ref(block_stmt));
                            }
                        }
                    }

                    self.env.exit_scope();
                    last = block_last;
                }
            }
        }

        last
    }
}


