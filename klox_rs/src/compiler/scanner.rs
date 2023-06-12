use super::tokenizer::{Token, TokenType};
use crate::vm::InterpreterError;
use phf::phf_map;
static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};
pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}
macro_rules! add_matched {
    ($scanner:ident, $to_match:expr, $expected:expr, $fallback:expr) => {{
        if $scanner.match_char($to_match) {
            return $scanner.make_token($expected);
        } else {
            return $scanner.make_token($fallback);
        }
    }};
}
pub type TokenError = Result<Token, InterpreterError>;

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;
        self.skip_whitespace();
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        };
        let c: char = self.advance();
        return match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '/' => self.make_token(TokenType::Slash),
            '+' => self.make_token(TokenType::Plus),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '!' => add_matched!(self, '=', TokenType::BangEqual, TokenType::Bang),
            '=' => add_matched!(self, '=', TokenType::EqualEqual, TokenType::Equal),
            '<' => add_matched!(self, '=', TokenType::LessEqual, TokenType::Less),
            '>' => add_matched!(self, '=', TokenType::GreaterEqual, TokenType::Greater),
            '"' => self.string_tok(),
            '0'..='9' => self.number_tok(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier_tok(),
            _ => self.error_token("Unexpected character."),
        };
    }
    fn error_token(&self, message: &str) -> Token {
        Token::new(TokenType::Error(message.to_string()), self.line).unwrap()
    }
    fn string_tok(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }
        self.advance();
        let value = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - 2)
            .collect();
        self.make_token(TokenType::String(value))
    }
    fn number_tok(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let value = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect::<String>()
            .parse::<f64>()
            .unwrap();
        self.make_token(TokenType::Number(value))
    }
    fn identifier_tok(&mut self) -> Token {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let value: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        if KEYWORDS.contains_key(value.as_str()) {
            return self.make_token(KEYWORDS.get(value.as_str()).cloned().unwrap());
        }
        self.make_token(TokenType::Identifier(value))
    }
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }
    fn is_at_end(&self) -> bool {
        self.source.chars().nth(self.current).unwrap_or('\0') == '\0'
    }
    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.line).unwrap()
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }
    fn skip_whitespace(&mut self) {
        loop {
            let c: char = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }
    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }
    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }
    fn make_error(&self, message: String) -> Token {
        Token::new(TokenType::Error(message), self.line).unwrap()
    }
}
