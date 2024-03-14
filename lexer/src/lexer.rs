use crate::token::{Literal, Token};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        use Token::*;

        match self.input.next() {
            Some(c) => Some(match c {
                ';' => SemiColon,
                ':' => Colon,
                ',' => Comma,
                '(' => LParen,
                ')' => RParen,
                '{' => LBrace,
                '}' => RBrace,
                '[' => LBracket,
                ']' => RBracket,
                '=' => Assign,
                '+' => Add,
                '-' => Sub,
                '*' => Mul,
                '/' => Div,
                '0'..='9' => {
                    let mut num = c.to_digit(10).unwrap() as isize;

                    while let Some(c) = self.input.next() {
                        if !c.is_numeric() {
                            break;
                        }

                        num = num * 10 + c.to_digit(10).unwrap() as isize;
                    }

                    Token::Lit(Literal::Number(num))
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut ident = String::new();
                    ident.push(c);

                    while let Some(c) = self.input.peek() {
                        if !c.is_alphanumeric() {
                            break;
                        }

                        ident.push(self.input.next().unwrap());
                    }

                    Identifier(ident)
                }
                '\"' => {
                    let mut lit = String::new();
                    while let Some(c) = self.input.next() {
                        if c == '\"' {
                            break;
                        }

                        lit.push(c);
                    }

                    Lit(Literal::String(lit))
                }
                '\'' => {
                    let tok = Token::Lit(Literal::Char(self.input.next().unwrap()));
                    self.input.next();

                    tok
                }
                c if c.is_ascii_whitespace() => return self.next_token(),
                _ => {
                    println!("Unknown token: {}", c);
                    todo!()
                }
            }),
            _ => None,
        }
    }
}
