#[derive(Debug, PartialEq)]

enum Token {
    Let,
    If,
    Else,
    Return,

    Identifier(String),
    Integer(i64),

    Equal,
    Plus,
    Minus,
    Star,
    Slash,

    EqualEqual,
    NotEqual,

    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    LeftBrace,
    RightBrace,

    Semicolon,

    EOF,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOF;
        }
        

        let ch = self.input[self.position];

        match ch {
            '=' => {
                if self.peek() == Some('=') {
                    self.position += 2;
                    Token::EqualEqual
                } else {
                    self.position += 1;
                    Token::Equal
                }
            }

            '!' => {
                if self.peek() == Some('=') {
                    self.position += 2;
                    Token::NotEqual
                } else {
                    self.position += 1;
                    Token::EOF
                }
            }

            '<' => {
                if self.peek() == Some('=') {
                    self.position += 2;
                    Token::LessEqual
                } else {
                    self.position += 1;
                    Token::Less
                }
            }

            '>' => {
                if self.peek() == Some('=') {
                    self.position += 2;
                    Token::GreaterEqual
                } else {
                    self.position += 1;
                    Token::Greater
                }
            }

            '+' => {
                self.position += 1;
                Token::Plus
            }

            '-' => {
                self.position += 1;
                Token::Minus
            }

            '*' => {
                self.position += 1;
                Token::Star
            }

            '/' => {
                self.position += 1;
                Token::Slash
            }

            ';' => {
                self.position += 1;
                Token::Semicolon
            }

            '{' => {
                self.position += 1;
                Token::LeftBrace
            }

            '}' => {
                self.position += 1;
                Token::RightBrace
            }

            ch if Self::is_identifier_start(ch) => {
                let identifier = self.read_identifier();
                Self::lookup_keyword(&identifier)
            }

            ch if ch.is_ascii_digit() => {
                let number = self.read_number();
                Token::Integer(number)
            }

            _ => {
                self.position += 1;
                Token::EOF
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
}

fn main() {
    let mut lexer = Lexer::new("let x = 10 + 20 * 3;

if x >= 50 {
    return x;
}");

    loop {
        let token = lexer.next_token();

        println!("{:?}", token);

        if token == Token::EOF {
            break;
        }
    }
}

