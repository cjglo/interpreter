use std::fmt;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    INTEGER(u32),
    OPERATION(char),
    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::INTEGER(x) => write!(f, "Integer {}", x),
            TokenType::OPERATION(x) => write!(f, "Operation {}", x),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}
