#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Var(String),
    Now,
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Stmt {
    Let { name: String, value: Expr },
    Expr(Expr),
}
