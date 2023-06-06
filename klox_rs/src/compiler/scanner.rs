use super::tokenizer::{Token, TokenType};
use crate::vm::InterpreterError;
use phf::phf_map;
static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::TokenAnd,
    "class" => TokenType::TokenClass,
    "else" => TokenType::TokenElse,
    "false" => TokenType::TokenFalse,
    "for" => TokenType::TokenFor,
    "fun" => TokenType::TokenFun,
    "if" => TokenType::TokenIf,
    "nil" => TokenType::TokenNil,
    "or" => TokenType::TokenOr,
    "print" => TokenType::TokenPrint,
    "return" => TokenType::TokenReturn,
    "super" => TokenType::TokenSuper,
    "this" => TokenType::TokenThis,
    "true" => TokenType::TokenTrue,
    "var" => TokenType::TokenVar,
    "while" => TokenType::TokenWhile,
};
struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}
macro_rules! add_matched {
    ($scanner:ident, $to_match:expr, $expected:expr, $fallback:expr) => {{
        if $scanner.match_char($to_match)? {
            return $scanner.make_token($expected);
        } else {
            return $scanner.make_token($fallback);
        }
    }};
}
pub type TokenError = Result<Token, InterpreterError>;

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }
    fn scan_token(&mut self) -> TokenError {
        self.start = self.current;
        self.skip_whitespace()?;
        if self.is_at_end() {
            return self.make_token(TokenType::TokenEof);
        };
        let c: char = self.advance()?;
        match c {
            '(' => self.make_token(TokenType::TokenLeftParen),
            ')' => self.make_token(TokenType::TokenRightParen),
            '{' => self.make_token(TokenType::TokenLeftBrace),
            '}' => self.make_token(TokenType::TokenRightBrace),
            ';' => self.make_token(TokenType::TokenSemicolon),
            ',' => self.make_token(TokenType::TokenComma),
            '.' => self.make_token(TokenType::TokenDot),
            '-' => self.make_token(TokenType::TokenMinus),
            '/' => self.make_token(TokenType::TokenSlash),
            '+' => self.make_token(TokenType::TokenPlus),
            '*' => self.make_token(TokenType::TokenStar),
            '!' => add_matched!(self, '=', TokenType::TokenBangEqual, TokenType::TokenBang),
            '=' => add_matched!(self, '=', TokenType::TokenEqualEqual, TokenType::TokenEqual),
            '<' => add_matched!(self, '=', TokenType::TokenLessEqual, TokenType::TokenLess),
            '>' => add_matched!(
                self,
                '=',
                TokenType::TokenGreaterEqual,
                TokenType::TokenGreater
            ),
            '"' => self.string_tok(),
            '0'..='9' => self.number_tok(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier_tok(),
            _ => return Err(InterpreterError::CompileError(self.line)),
        }
    }
    fn string_tok(&mut self) -> TokenError {
        while self.peek()? != '"' && !self.is_at_end() {
            if self.peek()? == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(InterpreterError::CompileError(self.line));
        }
        self.advance();
        let value = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - 2)
            .collect();
        return self.make_token(TokenType::TokenString(value));
    }
    fn number_tok(&mut self) -> TokenError {
        while self.peek()?.is_digit(10) {
            self.advance()?;
        }
        if self.peek()? == '.' && self.peek_next()?.is_digit(10) {
            self.advance()?;
            while self.peek()?.is_digit(10) {
                self.advance()?;
            }
        }
        let value = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect::<String>()
            .parse::<f64>()
            .map_err(|_| InterpreterError::CompileError(self.line))?;
        return self.make_token(TokenType::TokenNumber(value));
    }
    fn identifier_tok(&mut self) -> Result<Token, InterpreterError> {
        while self.peek()?.is_alphanumeric() || self.peek()? == '_' {
            self.advance()?;
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
        return self.make_token(TokenType::TokenIdentifier(value));
    }
    fn match_char(&mut self, expected: char) -> Result<bool, InterpreterError> {
        if self.is_at_end() {
            return Ok(false);
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return Ok(false);
        }
        self.current += 1;
        Ok(true)
    }
    fn is_at_end(&self) -> bool {
        self.source.chars().nth(self.current).unwrap() == '\0'
    }
    fn make_token(&self, token_type: TokenType) -> Result<Token, InterpreterError> {
        Token::new(token_type, self.line)
    }
    fn advance(&mut self) -> Result<char, InterpreterError> {
        self.current += 1;
        self.source
            .chars()
            .nth(self.current - 1)
            .ok_or(InterpreterError::EndOfFile)
    }
    fn skip_whitespace(&mut self) -> Result<(), InterpreterError> {
        loop {
            let c: char = self.peek()?;
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance()?;
                }
                '\n' => {
                    self.line += 1;
                    self.advance()?;
                }
                '/' => {
                    if self.peek_next()? == '/' {
                        while self.peek()? != '\n' && !self.is_at_end() {
                            self.advance()?;
                        }
                    } else {
                        return Ok(());
                    }
                }
                _ => return Ok(()),
            }
        }
    }
    fn peek(&self) -> Result<char, InterpreterError> {
        self.source
            .chars()
            .nth(self.current)
            .ok_or(InterpreterError::EndOfFile)
    }
    fn peek_next(&self) -> Result<char, InterpreterError> {
        self.source
            .chars()
            .nth(self.current + 1)
            .ok_or(InterpreterError::EndOfFile)
    }
}
