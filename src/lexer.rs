use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnexpectedCharacter(char),
    UnterminatedString,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Ok(Token::EOF);
        }
        

        let ch = self.input[self.position];

        match ch {
            '=' => {
                if self.peek() == Some('=') {
                    self.position += 2;
                    Ok(Token::EqualEqual)
                } else {
                    self.position += 1;
                    Ok(Token::Equal)
                }
            }

            '!' => {
                if self.peek() == Some('=') {
                    self.position += 2;
                    Ok(Token::NotEqual)
                } else {
                    self.position += 1;
                    Err(LexerError::UnexpectedCharacter('!'))
                }
            }

            '<' => {
                if self.peek() == Some('=') {
                    self.position += 2;
                    Ok(Token::LessEqual)
                } else {
                    self.position += 1;
                    Ok(Token::Less)
                }
            }

            '>' => {
                if self.peek() == Some('=') {
                    self.position += 2;
                    Ok(Token::GreaterEqual)
                } else {
                    self.position += 1;
                    Ok(Token::Greater)
                }
            }

            '+' => {
                self.position += 1;
                Ok(Token::Plus)
            }

            '-' => {
                self.position += 1;
                Ok(Token::Minus)
            }

            '*' => {
                self.position += 1;
                Ok(Token::Star)
            }

            '/' => {
                self.position += 1;
                Ok(Token::Slash)
            }

            ';' => {
                self.position += 1;
                Ok(Token::Semicolon)
            }

            '{' => {
                self.position += 1;
                Ok(Token::LeftBrace)
            }

            '}' => {
                self.position += 1;
                Ok(Token::RightBrace)
            }

            ch if Self::is_identifier_start(ch) => {
                let identifier = self.read_identifier();
                Ok(Self::lookup_keyword(&identifier))
            }

            ch if ch.is_ascii_digit() => {
                let number = self.read_number();
                Ok(Token::Integer(number))
            }

            '"' => {
                let value = self.read_string()?;
                Ok(Token::StringLiteral(value))
            }

            _ => {
                self.position += 1;
                Err(LexerError::UnexpectedCharacter(ch))
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;

        while self.position < self.input.len()
            && Self::is_identifier_part(self.input[self.position])
        {
            self.position += 1;
        }

        self.input[start..self.position]
            .iter()
            .collect()
    }

    fn lookup_keyword(identifier: &str) -> Token {
        match identifier {
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Identifier(identifier.to_string()),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self.input[self.position].is_whitespace()
        {
            self.position += 1;
        }
    }

    fn read_number(&mut self) -> i64 {
        let start = self.position;

        while self.position < self.input.len() 
            && self.input[self.position].is_ascii_digit()
        {
            self.position += 1;
        }

        let number: String = self.input[start..self.position]
            .iter()
            .collect();

        number.parse::<i64>().unwrap()
    }

    fn is_identifier_start(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }

    fn is_identifier_part(ch: char) -> bool {
        ch.is_ascii_alphanumeric() || ch == '_'
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn read_string(&mut self) -> Result<String, LexerError> {
        self.position += 1; //skip opining "
        
        let start = self.position;

        while self.position < self.input.len()
            && self.input[self.position] != '"' 
            {
                self.position += 1;
            }

            if self.position >= self.input.len() {
                return Err(LexerError::UnterminatedString);
            }

            let value: String = self.input[start..self.position]
                .iter()
                .collect();

            self.position += 1; // skip closing "

            Ok(value)
    }
}