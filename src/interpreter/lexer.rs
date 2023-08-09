use super::token::TokenType;
use std::fmt::Debug;

#[derive(Debug)]
pub enum ParserError {
    UnrecognizedToken(String),
}

pub struct Lexer {
    text: String,

    pos: usize,

    current_char: Option<char>,
}

impl Lexer {
    pub fn new(text: String, pos: usize) -> Lexer {
        Lexer {
            text,
            pos,
            current_char: Some('\0'),
        }
    }

    pub fn get_next_token(&mut self) -> Result<TokenType, ParserError> {
        if self.pos >= self.text.len() {
            return Ok(TokenType::EOF);
        }

        const ERROR_MESSAGE: &str = "Unrecognized token: ";

        self.current_char = self.text.chars().nth(self.pos);

        while let Some(c) = self.current_char {
            if c == ' ' {
                self.pos += 1;

                self.current_char = self.text.chars().nth(self.pos);

                continue;
            }

            if c.is_digit(10) {
                return Ok(TokenType::INTEGER(self.parse_integer(c)));
            }

            if c == '+' {
                self.pos += 1;
                return Ok(TokenType::OPERATION(c));
            }

            if c == '-' {
                self.pos += 1;
                return Ok(TokenType::OPERATION(c));
            }

            if c == '*' {
                self.pos += 1;
                return Ok(TokenType::OPERATION(c));
            }

            if c == '/' {
                self.pos += 1;
                return Ok(TokenType::OPERATION(c));
            }

            return Err(ParserError::UnrecognizedToken(
                ERROR_MESSAGE.to_owned() + &c.to_string(),
            ));
        }
        Err(ParserError::UnrecognizedToken(
            ERROR_MESSAGE.to_owned() + &self.current_char.unwrap().to_string(),
        ))
    }

    fn parse_integer(&mut self, c: char) -> u32 {
        let mut sum = 0;
        let mut current_char = c;

        while current_char.is_digit(10) {
            sum *= 10;
            sum += current_char.to_digit(10).unwrap();

            if self.pos + 1 < self.text.len() {
                self.pos += 1;

                current_char = self.text.chars().nth(self.pos).unwrap();
            } else {
                self.pos += 1;

                break;
            }
        }

        sum
    }
}
