#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,

    // Identifiers & literals
    Ident(String),
    Number(i64),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,

    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> i64 {
        let mut n = String::new();
        while let Some(c) = self.current() {
            if c.is_ascii_digit() {
                n.push(c);
                self.advance();
            } else {
                break;
            }
        }
        n.parse().unwrap()
    }

    fn read_ident(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.current() {
            if c.is_alphanumeric() || c == '_' {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        s
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current() {
            Some('+') => { self.advance(); Token::Plus }
            Some('-') => { self.advance(); Token::Minus }
            Some('*') => { self.advance(); Token::Star }
            Some('/') => { self.advance(); Token::Slash }
            Some('=') => { self.advance(); Token::Equal }
            Some(c) if c.is_ascii_digit() => Token::Number(self.read_number()),
            Some(c) if c.is_alphabetic() || c == '_' => {
                let ident = self.read_ident();
                if ident == "let" {
                    Token::Let
                } else {
                    Token::Ident(ident)
                }
            }
            None => Token::EOF,
            Some(c) => panic!("Unexpected character: {}", c),
        }
    }
}

