use crate::chunk::OpCode;
use crate::compiler::scanner::Scanner;
use crate::value::Value;
use crate::{chunk::Chunk, vm::InterpreterError};

use self::scanner::TokenError;
use self::tokenizer::{Precendence, Token, TokenType};

mod parse_rule;
mod scanner;
mod tokenizer;
pub struct Compiler<'a> {
    chunk: &'a mut Chunk,
    scanner: Scanner,
    parser: Parser,
}
struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panicMode: bool,
}
impl Parser {
    pub fn new() -> Parser {
        Parser {
            current: Token::default(),
            previous: Token::default(),
            had_error: false,
            panicMode: false,
        }
    }
}
impl Compiler<'_> {
    pub fn new(source: String, chunk: &'_ mut Chunk) -> Compiler {
        Compiler {
            chunk,
            scanner: Scanner::new(source),
            parser: Parser::new(),
        }
    }
    pub fn compile(&mut self) -> Result<(), InterpreterError> {
        self.advance().unwrap();
        self.expression()?;
        self.consume(TokenType::Eof, "Expect end of expression.");
        self.end_compiler();
        Ok(())
    }
    fn expression(&mut self) -> Result<(), InterpreterError> {
        self.parser_precedence(Precendence::Assignment);
        Ok(())
    }
    fn advance(&mut self) -> TokenError {
        self.parser.previous = self.parser.current.clone();
        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.token_type().clone() != TokenType::Error("L".into()) {
                break Ok(self.parser.current.clone());
            }
        }
    }
    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.parser.current.token_type().clone() == token_type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }
    fn emit_byte(&mut self, byte: OpCode) {
        self.chunk.write_chunk(byte, self.parser.previous.line());
    }
    fn emit_bytes(&mut self, byte1: OpCode, byte2: OpCode) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }
    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn);
    }
    fn emit_constant(&mut self, value: Value) {
        self.chunk
            .write_constant(value, self.parser.previous.line());
    }
    fn end_compiler(&mut self) {
        self.emit_return();
    }
    fn parser_precedence(&mut self, token_type: Precendence) {
        self.advance();
        let prefix_rule = parse_rule::get_rule(self.parser.previous.token_type().clone()).prefix;
        if let Some(prefix_rule) = prefix_rule {
            prefix_rule(self);
            while token_type
                <= parse_rule::get_rule(self.parser.current.token_type().clone()).precedence
            {
                self.advance();
                let infix_rule = parse_rule::get_rule(self.parser.previous.token_type().clone())
                    .infix
                    .unwrap();
                infix_rule(self);
            }
        } else {
            self.error("Expect expression")
        }
    }
    fn error_at(&mut self, token: Token, message: &str) {
        if self.parser.panicMode {
            return;
        };
        self.parser.panicMode = true;
        print!("Error at line {} : {}", token.line(), message);
        if token.token_type() == &TokenType::Eof {
            print!(" at end");
        } else if token.token_type() == &TokenType::Error("L".into()) {
            // Nothing.
        } else {
            print!(" at '{:?}'", token.token_type());
        }
    }
    fn error(&mut self, message: &str) {
        self.error_at(self.parser.previous.clone(), message);
    }
    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.parser.current.clone(), message);
    }
}
