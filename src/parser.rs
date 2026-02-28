use crate::lexer::{Lexer, Token};
use crate::ast::*;

pub struct Parser {
    lexer: Lexer,
    current: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        Self { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current != Token::EOF {
            stmts.push(self.parse_stmt());
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.current {
            Token::Let => self.parse_let(),
            _ => Stmt::Expr(self.parse_expr()),
        }
    }

    fn parse_let(&mut self) -> Stmt {
        self.advance(); // let

        let name = match &self.current {
            Token::Ident(s) => s.clone(),
            _ => panic!("Expected identifier"),
        };
        self.advance();

        if self.current != Token::Equal {
            panic!("Expected '='");
        }
        self.advance();

        let value = self.parse_expr();
        Stmt::Let { name, value }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_add()
    }

    fn parse_add(&mut self) -> Expr {
        let mut expr = self.parse_mul();
        loop {
            match self.current {
                Token::Plus => {
                    self.advance();
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinOp::Add,
                        right: Box::new(self.parse_mul()),
                    };
                }
                Token::Minus => {
                    self.advance();
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinOp::Sub,
                        right: Box::new(self.parse_mul()),
                    };
                }
                _ => break,
            }
        }
        expr
    }

    fn parse_mul(&mut self) -> Expr {
        let mut expr = self.parse_primary();
        loop {
            match self.current {
                Token::Star => {
                    self.advance();
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinOp::Mul,
                        right: Box::new(self.parse_primary()),
                    };
                }
                Token::Slash => {
                    self.advance();
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinOp::Div,
                        right: Box::new(self.parse_primary()),
                    };
                }
                _ => break,
            }
        }
        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match &self.current {
            Token::Number(n) => {
                let v = *n;
                self.advance();
                Expr::Number(v)
            }
            Token::Ident(s) => {
                let ident = s.clone();
                self.advance();
                if ident == "now" {
                    Expr::Now
                } else {
                    Expr::Var(ident)
                }
            }
            _ => panic!("Unexpected token"),
        }
    }
}
