use super::{
    tokenizer::{Precendence, TokenType},
    Compiler,
};
use crate::{chunk::OpCode, vm::InterpreterError};
pub type ParseFn = fn(&mut Compiler) -> Result<(), InterpreterError>;
pub struct ParseRule {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Precendence,
}

pub fn uranary(compiler: &mut Compiler) -> Result<(), InterpreterError> {
    let operator_type = compiler.parser.previous.token_type().clone();
    compiler.parser_precedence(Precendence::Unary);
    match operator_type {
        TokenType::Minus => compiler.emit_byte(OpCode::OpNegate),
        _ => Err(InterpreterError::SourceError)?,
    };
    Ok(())
}
fn grouping(compiler: &mut Compiler) -> Result<(), InterpreterError> {
    compiler.expression()?;
    compiler.consume(TokenType::RightParen, "Expect ')' after expression.");
    Ok(())
}
fn binary(compiler: &mut Compiler) -> Result<(), InterpreterError> {
    let operator_type = compiler.parser.previous.token_type().clone();
    let rule = get_rule(operator_type.clone());
    compiler.parser_precedence(rule.precedence.increment());
    match operator_type {
        TokenType::Plus => compiler.emit_byte(OpCode::OpAdd),
        TokenType::Minus => compiler.emit_byte(OpCode::OpSubtract),
        TokenType::Star => compiler.emit_byte(OpCode::OpMultiply),
        TokenType::Slash => compiler.emit_byte(OpCode::OpDivide),
        _ => Err(InterpreterError::SourceError)?,
    };
    Ok(())
}
fn number(compiler: &mut Compiler) -> Result<(), InterpreterError> {
    if let TokenType::Number(value) = compiler.parser.previous.token_type() {
        compiler.emit_constant(*value);
    } else {
        return Err(InterpreterError::SourceError);
    }
    Ok(())
}
pub fn get_rule(token: TokenType) -> ParseRule {
    match token {
        TokenType::LeftParen => ParseRule {
            prefix: Some(grouping),
            infix: None,
            precedence: Precendence::None,
        },
        TokenType::Minus => ParseRule {
            prefix: Some(uranary),
            infix: Some(binary),
            precedence: Precendence::Term,
        },
        TokenType::Plus => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precendence::Term,
        },
        TokenType::Slash | TokenType::Star => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precendence::Factor,
        },

        TokenType::Number(_) => ParseRule {
            prefix: Some(number),
            infix: None,
            precedence: Precendence::None,
        },
        TokenType::And
        | TokenType::RightParen
        | TokenType::LeftBrace
        | TokenType::RightBrace
        | TokenType::Comma
        | TokenType::Dot
        | TokenType::Class
        | TokenType::Else
        | TokenType::False
        | TokenType::Fun
        | TokenType::For
        | TokenType::If
        | TokenType::Nil
        | TokenType::Or
        | TokenType::Print
        | TokenType::Semicolon
        | TokenType::Return
        | TokenType::Super
        | TokenType::This
        | TokenType::True
        | TokenType::Var
        | TokenType::While
        | TokenType::Error(_)
        | TokenType::Eof
        | TokenType::Bang
        | TokenType::BangEqual
        | TokenType::Equal
        | TokenType::EqualEqual
        | TokenType::Greater
        | TokenType::GreaterEqual
        | TokenType::Less
        | TokenType::LessEqual
        | TokenType::Identifier(_)
        | TokenType::String(_) => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },
    }
}
