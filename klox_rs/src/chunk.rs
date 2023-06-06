#![allow(dead_code)]
use std::fmt::Display;

use crate::value::ValueArray;
#[derive(Debug)]
pub enum OpCode {
    OpConstant(usize),
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpReturn,
}
impl Copy for OpCode {}
impl Clone for OpCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::OpReturn => write!(f, "OpReturn"),
            OpCode::OpConstant(i) => write!(f, "OpConstant {}", i),
            OpCode::OpNegate => write!(f, "OpNegate"),
            OpCode::OpAdd => write!(f, "OpAdd"),
            OpCode::OpSubtract => write!(f, "OpSubtract"),
            OpCode::OpMultiply => write!(f, "OpMultiply"),
            OpCode::OpDivide => write!(f, "OpDivide"),
        }
    }
}
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub linenum: Vec<usize>,
    pub constants: ValueArray,
}
impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            linenum: Vec::new(),
            constants: ValueArray::new(),
        }
    }
    pub fn write_chunk(&mut self, byte: OpCode, line: usize) {
        self.code.push(byte);
        self.linenum.push(line);
    }
    pub fn add_constant(&mut self, value: f64) -> usize {
        self.constants.write(value)
    }
    pub fn write_constant(&mut self, value: f64, line: usize) {
        let index = self.add_constant(value);
        self.write_chunk(OpCode::OpConstant(index), line);
    }
    pub fn write_raw_constant(&mut self, index: usize, line: usize) {
        self.write_chunk(OpCode::OpConstant(index), line);
    }
    pub fn disassemble(&self, name: &str) -> String {
        let mut result = String::new();
        result.push_str(&format!("== {} ==\n", name));
        for (i, byte) in self.code.iter().enumerate() {
            result.push_str(&format!("{:04} ", self.linenum[i]));
            result.push_str(self.disassemble_instruction(byte, i).as_str());
        }
        result
    }
    pub fn read_constant(&self, index: usize) -> f64 {
        self.constants.values[index]
    }
    fn disassemble_instruction(&self, byte: &OpCode, offset: usize) -> String {
        match byte {
            OpCode::OpReturn => self.simple_instruction("OP_RETURN", offset),
            OpCode::OpNegate => self.simple_instruction("OP_NEGATE", offset),
            OpCode::OpAdd => self.simple_instruction("OP_ADD", offset),
            OpCode::OpSubtract => self.simple_instruction("OP_SUBTRACT", offset),
            OpCode::OpMultiply => self.simple_instruction("OP_MULTIPLY", offset),
            OpCode::OpDivide => self.simple_instruction("OP_DIVIDE", offset),
            OpCode::OpConstant(i) => {
                let value = self.constants.values[*i];
                self.constant_instruction("OP_CONSTANT", value, offset)
            }
        }
    }
    fn simple_instruction(&self, name: &str, offset: usize) -> String {
        format!("{} {}\n", name, offset)
    }
    fn constant_instruction(&self, name: &str, value: f64, offset: usize) -> String {
        format!("{} {} '{}'\n", name, offset, value)
    }
}
impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
