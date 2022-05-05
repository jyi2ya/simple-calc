use std::io;

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(f64),
    Operator(char),
    Empty,
    End,
}

impl Token {
    fn get_number(&self) -> Option<f64> {
        match self {
            Token::Number(number) => Some(*number),
            _ => None,
        }
    }
}

struct Lexer {
    raw: String,
}

pub trait Scan {
    fn next(&mut self) -> Token;
}

impl Lexer {
    fn new(s: String) -> Self {
        Lexer {
            raw: s
        }
    }
}

impl Scan for Lexer {
    fn next(&mut self) -> Token {
        let s = (&self.raw[..]).trim_start();
        let first = match s.chars().next() {
            Some(ch) => ch,
            None => return Token::End,
        };

        match first {
            ch if ch.is_ascii_digit() => {
                let idx = s.find(|c: char| ! c.is_ascii_digit()).unwrap();
                let number = (&s[..idx]).parse().unwrap();
                self.raw = (&s[idx..]).to_string();
                Token::Number(number)
            },

            ch if matches!(ch, '+'|'-'|'*'|'/'|'%'|'('|')') => {
                let operator = s.chars().next().unwrap();
                self.raw = (&s[1..]).to_string();
                Token::Operator(operator)
            },

            _ => panic!(),
        }
    }
}

mod parser {
    use crate::{Token, Scan};

    type Result<T> = std::result::Result<T, &'static str>;

    pub struct Parser<'a, T> {
        lexer: &'a mut T,
        current: Token,
        look_ahead: Token,
    }

    impl<'a, T: Scan> Parser<'a, T> {
        pub fn new(lexer: &'a mut T) -> Self {
            Parser {
                lexer,
                current: Token::Empty,
                look_ahead: Token::Empty,
            }
        }

        fn shift(&mut self) -> Token {
            let result = self.current;
            self.current = self.look_ahead;
            self.look_ahead = self.lexer.next();
            return result;
        }

        fn eval_primary_expr(&mut self) -> Result<f64> {
            match self.current {
                Token::Operator('(') => {
                    self.shift();
                    let result = self.eval_add_expr()?;

                    if let Token::Operator(')') = self.look_ahead {
                        self.current = self.shift();
                        Ok(result)
                    } else {
                        Err("unmatched bracket")
                    }
                },

                Token::Number(number) => Ok(number),

                _ => Err("invalid operator"),
            }
        }

        fn eval_unary_expr(&mut self) -> Result<f64> {
            match self.current {
                Token::Operator('+') | Token::Operator('-') => {
                    let operator = self.shift();
                    let oprand = self.eval_primary_expr()?;
                    let result = match operator {
                        Token::Operator('+') => oprand,
                        Token::Operator('-') => - oprand,
                        _ => unreachable!(),
                    };

                    self.current = Token::Number(result);
                    Ok(result)
                },
                _ => self.eval_primary_expr(),
            }
        }

        fn eval_mul_expr(&mut self) -> Result<f64> {
            self.eval_unary_expr()?;

            match self.look_ahead {
                Token::Operator('*') | Token::Operator('/') => {
                    let op1 = self.shift().get_number().unwrap();

                    let operator = self.shift();
                    let op2 = self.eval_unary_expr()?;

                    let result = match operator {
                        Token::Operator('*') => op1 * op2,
                        Token::Operator('/') => op1 / op2,
                        _ => unreachable!(),
                    };

                    self.current = Token::Number(result);

                    self.eval_mul_expr()
                },

                _ => if let Token::Number(result) = self.current {
                    Ok(result)
                } else {
                    Err("error occurred")
                }
            }
        }

        fn eval_add_expr(&mut self) -> Result<f64> {
            self.eval_mul_expr()?;

            match self.look_ahead {
                Token::Operator('+') | Token::Operator('-') => {
                    let op1 = self.shift().get_number().unwrap();

                    let operator = self.shift();
                    let op2 = self.eval_mul_expr()?;

                    let result = match operator {
                        Token::Operator('+') => op1 + op2,
                        Token::Operator('-') => op1 - op2,
                        _ => unreachable!(),
                    };

                    self.current = Token::Number(result);

                    self.eval_add_expr()
                },

                _ => if let Token::Number(result) = self.current {
                    Ok(result)
                } else {
                    Err("error occurred")
                }
            }
        }

        pub fn eval(&mut self) -> Result<f64> {
            self.shift();
            self.shift();
            let result = self.eval_add_expr();
            if let Token::End = self.look_ahead {
                result
            } else {
                Err("invalid expression")
            }
        }
    }
}

use parser::Parser;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            break
        }

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let result = parser.eval();

        match result {
            Ok(result) => println!("{}", result),
            Err(msg) => println!("{}", msg),
        }
    }
}
