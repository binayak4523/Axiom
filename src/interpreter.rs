use std::collections::HashMap;
use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Time(i64),
}

#[derive(Debug)]
pub struct Env {
    vars: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.vars.get(name).cloned()
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
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
                    self.env.set(name.clone(), val);
                }
                Stmt::Expr(expr) => {
                    last = Some(self.eval_expr(expr));
                }
            }
        }

        last
    }
}


