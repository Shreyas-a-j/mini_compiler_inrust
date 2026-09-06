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
                self.position += 1;
                Token::Equal
            }

            '+' => {
                self.position += 1;
                Token::Plus
            }

            ';' => {
                self.position += 1;
                Token::Semicolon
            }

            ch if ch.is_alphabetic() || ch == '_' => {
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
            && (self.input[self.position].is_alphanumeric()
                || self.input[self.position] == '_')
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
}

fn main() {
    let mut lexer = Lexer::new("let x = 10 + 20;");

    loop {
        let token = lexer.next_token();

        println!("{:?}", token);

        if token == Token::EOF {
            break;
        }
    }
}

