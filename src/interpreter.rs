mod token;
mod lexer;

use token::TokenType;
use lexer::{Lexer, ParserError};

pub struct Interpreter {
    lexer: Lexer,
    current_token: TokenType,
}

impl Interpreter {
    const ERROR_MESSAGE: &str = "Unrecognized token: ";

    pub fn new(text: String, pos: usize) -> Interpreter {
        let mut interpreter = Interpreter {
            lexer: Lexer::new(text, pos),
            current_token: TokenType::EOF,
        };
        interpreter.current_token = interpreter.lexer.get_next_token().unwrap();

        interpreter
    }

    fn factor(&mut self) -> Result<i64, ParserError> {
        // * An Integer

        if let TokenType::INTEGER(x) = self.current_token {
            self.current_token = self.lexer.get_next_token().unwrap();
            Ok(x as i64)
        } else {
            Err(ParserError::UnrecognizedToken(
                Self::ERROR_MESSAGE.to_owned() + &self.current_token.to_string(),
            ))
        }
    }

    fn term(&mut self) -> Result<i64, ParserError> {
        // * factor ((Mult | Div) factor)

        let mut result = self.factor().unwrap();

        while let TokenType::OPERATION(op) = self.current_token {

            if op == '*' {
                self.current_token = self.lexer.get_next_token().unwrap();
                result *= self.factor().unwrap();

            } else if op == '/' {
                self.current_token = self.lexer.get_next_token().unwrap();
                result /= self.factor().unwrap();
            } else {
                break;
            }
        }

        Ok(result)
    }

    pub fn expr(&mut self) -> Result<i64, ParserError> {
        /*
        ====== Arithmetic expression parser / interpreter. ======
        expr   : term ((PLUS | MINUS) term)*
        term   : factor ((MUL | DIV) factor)*
        factor : INTEGER
        =========================================================
        */

        let mut result = self.term().unwrap();

        while let TokenType::OPERATION(op) = self.current_token {

            if op == '+' {
                self.current_token = self.lexer.get_next_token().unwrap();
                result += self.term().unwrap();
            } else if op == '-' {
                self.current_token = self.lexer.get_next_token().unwrap();
                result -= self.term().unwrap();
            } else {
                break;
            }
        }

        return Ok(result);
    }
}
