use crate::error::Error;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Scanner<'a> {
    error: &'a mut Error,
    program: String,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("fun", TokenType::Fun);
        m.insert("if", TokenType::If);
        m.insert("nil", TokenType::Nil);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("true", TokenType::True);
        m.insert("var", TokenType::Var);
        m.insert("while", TokenType::While);
        m
    };
}

impl<'a> Scanner<'a> {
    pub fn new(program: String, error: &'a mut Error) -> Self {
        Self {
            error,
            program,
            tokens: vec![],
            current: 0,
            start: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => self.add_conditional_token('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.add_conditional_token('=', TokenType::Equal, TokenType::EqualEqual),
            '<' => self.add_conditional_token('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.add_conditional_token('=', TokenType::GreaterEqual, TokenType::Greater),
            '/' => {
                if self.consume_expected('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {
                // noop
            }
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_ascii_alphabetic() {
                    self.identifier()
                } else {
                    self.error
                        .error(self.line, format!("Unexpected character {}.", c).as_str());
                }
            }
        }
    }

    fn add_conditional_token(&mut self, expected: char, matched: TokenType, unmatched: TokenType) {
        let token_type = if self.consume_expected(expected) {
            matched
        } else {
            unmatched
        };
        self.add_token(token_type);
    }

    fn consume_expected(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != expected {
            return false;
        }
        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        self.program.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current > self.program.len() {
            '\0'
        } else {
            self.program.chars().nth(self.current + 1).unwrap()
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.program.len()
    }

    fn advance(&mut self) -> char {
        let c = self.program.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None)
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = &self.program[self.start..self.current];
        let lexeme = text.to_string();
        let token = Token::new(token_type, lexeme, literal, self.line);
        self.tokens.push(token);
    }

    fn string(&mut self) {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        if self.is_at_end() {
            self.error.error(self.line, "Unterminated string.");
            return;
        }
        self.advance();
        let text = &self.program[self.start + 1..self.current - 1];
        let lexeme = text.to_string();
        let literal = Literal::String(lexeme);
        self.add_token_literal(TokenType::String, Some(literal));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() && !self.is_at_end() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() && !self.is_at_end() {
                self.advance();
            }
        }
        let number = Literal::Number(
            self.program[self.start..self.current]
                .parse::<f64>()
                .unwrap(),
        );
        self.add_token_literal(TokenType::Number, Some(number));
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() && !self.is_at_end() {
            self.advance();
        }
        let text = &self.program[self.start..self.current];
        let token_type = KEYWORDS.get(text).unwrap_or(&TokenType::Identifier);
        self.add_token(*token_type);
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str())
    }
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    fn str(&self) -> String {
        format!(
            "[Line {}] Type = {:?}, Lexeme = {}, literal = {:?}",
            self.line, self.token_type, self.lexeme, self.literal
        )
    }
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
}

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
