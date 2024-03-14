use std::isize;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(isize),
    String(String),
    Char(char),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Lit(Literal),
    Colon,
    Comma,
    SemiColon,
    Assign,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Mathematical Operations
    Add,
    Sub,
    Mul,
    Div,
}
