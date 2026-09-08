use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnexpectedCharacter(char),
    UnterminatedString,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        loop {
            self.skip_whitespace();

            if self.position >= self.input.len() {
                return Ok(Token::EOF);
            }
        

            let ch = self.input[self.position];

            match ch {
                '=' => {
                    if self.peek() == Some('=') {
                        self.position += 2;
                        return Ok(Token::EqualEqual)
                    } else {
                        self.position += 1;
                        return Ok(Token::Equal)
                    }
                }

                '!' => {
                    if self.peek() == Some('=') {
                        self.position += 2;
                        return Ok(Token::NotEqual)
                    } else {
                        self.position += 1;
                        return Err(LexerError::UnexpectedCharacter('!'))
                    }
                }

                '<' => {
                    if self.peek() == Some('=') {
                        self.position += 2;
                        return Ok(Token::LessEqual)
                    } else {
                        self.position += 1;
                        return Ok(Token::Less)
                    }
                }

                '>' => {
                    if self.peek() == Some('=') {
                        self.position += 2;
                        return Ok(Token::GreaterEqual)
                    } else {
                        self.position += 1;
                        return Ok(Token::Greater)
                    }
                }

                '+' => {
                    self.position += 1;
                    return Ok(Token::Plus)
                }

                '-' => {
                    self.position += 1;
                    return Ok(Token::Minus)
                }

                '*' => {
                    self.position += 1;
                    return Ok(Token::Star)
                }

                '/' => {
                    if self.peek() == Some('/') {
                        self.position += 2;
                        self.skip_comment();

                        let _ = self.next_token();
                    } else {
                        self.position += 1;
                        return Ok(Token::Slash)
                    }
                }

                ';' => {
                    self.position += 1;
                    return Ok(Token::Semicolon)
                }

                '{' => {
                    self.position += 1;
                    return Ok(Token::LeftBrace)
                }

                '}' => {
                    self.position += 1;
                    return Ok(Token::RightBrace)
               }

                ch if Self::is_identifier_start(ch) => {
                    let identifier = self.read_identifier();
                    return Ok(Self::lookup_keyword(&identifier))
                }

                ch if ch.is_ascii_digit() => {
                    let number = self.read_number();
                    return Ok(Token::Integer(number))
                }

                '"' => {
                    let value = self.read_string()?;
                    return Ok(Token::StringLiteral(value))
                }

                _ => {
                    self.position += 1;
                    return Err(LexerError::UnexpectedCharacter(ch))
                }
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

    fn skip_comment(&mut self) {
        while self.position < self.input.len()
            && self.input[self.position] != '\n'
            {
                self.position += 1;
            }
    }

    fn advance(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            return None;
        }

        let ch = self.input[self.position];

        self.position += 1;

        if ch == '\n' {
            self.line += 1;
            self.column += 1;
        } else {
            self.column += 1;
        }

        Some(ch)
    }
}