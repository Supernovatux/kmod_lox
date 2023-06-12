use super::scanner::TokenError;
use derivative::Derivative;
#[derive(Derivative)]
#[derivative(Clone, Debug, PartialEq, Hash)]
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
    Identifier(String),
    String(String),
    Number(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        f64,
    ),
    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
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

    Error(#[derivative(PartialEq = "ignore")] String),
    Eof,
}
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Precendence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}
impl Precendence {
    pub fn increment(&self) -> Precendence {
        match self {
            Precendence::None => Precendence::Assignment,
            Precendence::Assignment => Precendence::Or,
            Precendence::Or => Precendence::And,
            Precendence::And => Precendence::Equality,
            Precendence::Equality => Precendence::Comparison,
            Precendence::Comparison => Precendence::Term,
            Precendence::Term => Precendence::Factor,
            Precendence::Factor => Precendence::Unary,
            Precendence::Unary => Precendence::Call,
            Precendence::Call => Precendence::Primary,
            Precendence::Primary => Precendence::Primary,
        }
    }
}
#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    line: usize,
}
impl Token {
    pub fn new(token_type: TokenType, line: usize) -> TokenError {
        Ok(Token { token_type, line })
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn line(&self) -> usize {
        self.line
    }
}
impl Default for Token {
    fn default() -> Token {
        Token {
            token_type: TokenType::Nil,
            line: 0,
        }
    }
}
