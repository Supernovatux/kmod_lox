use std::fmt::Debug;

use crate::{
    chunk::{Chunk, OpCode},
    compiler::compile,
    value::Value,
};
#[derive(Debug)]
pub enum InterpreterError {
    CompileError(usize),
    RuntimeError(usize),
    ScanningError(usize),
    SourceError,
    EndOfFile,
}
impl std::error::Error for InterpreterError {}
pub enum InterpreterStatus {
    Running,
    Returning,
}
impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::CompileError(line) => write!(f, "Compile error at line {}", line),
            InterpreterError::RuntimeError(line) => write!(f, "Runtime error at line {}", line),
            InterpreterError::SourceError => write!(f, "Source error"),
            InterpreterError::ScanningError(line) => write!(f, "Scanning error at line {}", line),
            InterpreterError::EndOfFile => write!(f, "End of file"),
        }
    }
}
pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    stack_top: usize,
}
//Binary op macro
macro_rules! binary_op {
    ($vm:ident, $op:tt) => {{
        let b = $vm.pop()?;
        let a = $vm.pop()?;
        $vm.push(a $op b);
    }};
}
impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::new(),
            stack_top: 0,
        }
    }
    pub fn interpret(&mut self, source: String) -> Result<(), InterpreterError> {
        compile(source);
        Ok(())
    }
    fn run(&mut self) -> Result<(), InterpreterError> {
        loop {
            let instruction = self.read_byte()?;
            if matches!(
                self.interpret_opcode(instruction)?,
                InterpreterStatus::Returning
            ) {
                break;
            }
        }
        Ok(())
    }

    fn interpret_opcode(
        &mut self,
        instruction: OpCode,
    ) -> Result<InterpreterStatus, InterpreterError> {
        match instruction {
            OpCode::OpReturn => {
                println!("{}", self.pop()?);
                return Ok(InterpreterStatus::Returning);
            }
            OpCode::OpAdd => binary_op!(self, +),
            OpCode::OpSubtract => binary_op!(self, -),
            OpCode::OpMultiply => binary_op!(self, *),
            OpCode::OpDivide => binary_op!(self, /),
            OpCode::OpNegate => {
                let value = self.pop()?;
                self.push(-value);
            }
            OpCode::OpConstant(index) => {
                let constant = self.read_constant(index);
                self.push(constant);
            }
        }
        Ok(InterpreterStatus::Running)
    }
    fn read_byte(&mut self) -> Result<OpCode, InterpreterError> {
        self.ip += 1;
        if self.ip > self.chunk.code.len() {
            return Err(InterpreterError::CompileError(
                self.chunk.linenum[self.ip - 1],
            ));
        }
        Ok(self.chunk.code[self.ip - 1])
    }
    fn read_constant(&self, index: usize) -> f64 {
        self.chunk.read_constant(index)
    }
    pub fn push(&mut self, value: Value) -> usize {
        self.stack.push(value);
        self.stack_top += 1;
        self.stack_top
    }
    pub fn pop(&mut self) -> Result<Value, InterpreterError> {
        self.stack_top -= 1;
        self.stack.pop().ok_or(InterpreterError::RuntimeError(
            self.chunk.linenum[self.ip - 1],
        ))
    }
}
impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
