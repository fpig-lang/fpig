use utils::op::OpCode;
use vm::chunk::Chunk;

use crate::ast::{Binaryop, Expr, ExprKind, ParseObj, Unaryop};

pub struct Compiler {
    chunk: Chunk,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            chunk: Chunk::new(),
        }
    }

    pub fn compile(&mut self, ast: Expr) {
        self.compile_expr(ast);
    }

    fn compile_expr(&mut self, expr: Expr) {
        match expr.node {
            ExprKind::Binary { left, op, right } => {
                self.compile_expr(*left);
                self.compile_expr(*right);
                self.emit_binary_op(op);
            }
            ExprKind::Group { body } => {
                self.compile_expr(*body);
            }
            ExprKind::Literal { value } => {
                self.compile_literal(value);
            }
            ExprKind::Unary { op, operand } => {
                self.compile_expr(*operand);
                self.emit_unaryop(op);
            }
        }
    }

    fn emit(&mut self, code: u8) {
        self.chunk.write_code(code);
    }

    fn emit_binary_op(&mut self, op: Binaryop) {
        match op {
            Binaryop::Add => self.emit(OpCode::Add as u8),
            Binaryop::Sub => self.emit(OpCode::Sub as u8),
            Binaryop::Mult => self.emit(OpCode::Mult as u8),
            Binaryop::Div => self.emit(OpCode::Div as u8),
            Binaryop::Eq => self.emit(OpCode::Eq as u8),
            Binaryop::NotEq => {
                self.emit(OpCode::Eq as u8);
                self.emit(OpCode::Not as u8);
            }
            Binaryop::Gt => self.emit(OpCode::Gt as u8),
            Binaryop::GtE => todo!(),
            Binaryop::Lt => self.emit(OpCode::Lt as u8),
            Binaryop::LtE => todo!(),
            Binaryop::And => todo!(),
            Binaryop::Or => todo!(),
        }
    }

    fn compile_literal(&mut self, value: ParseObj) {
        match value {
            ParseObj::Nil => self.emit(OpCode::Nil as u8),
            ParseObj::Bool(_) => todo!(),
            ParseObj::Int(_) => todo!(),
            ParseObj::Float(_) => todo!(),
            ParseObj::Str(_) => todo!(),
            ParseObj::Ident(_) => todo!(),
        }
    }

    fn emit_unaryop(&mut self, op: Unaryop) {
        match op {
            Unaryop::Not => self.emit(OpCode::Not as u8),
            Unaryop::Minus => todo!(),
        }
    }
}
