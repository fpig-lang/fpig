use std::collections::HashMap;

use vm::{chunk::Chunk, op::OpCode, value::Value};

use crate::ast::{Binaryop, Expr, ExprKind, ParseObj, Unaryop, Stmt, StmtKind};

pub(crate) struct Compiler {
    chunk: Chunk,
    global: HashMap<String, u16>,
}

impl Compiler {
    pub(crate) fn new() -> Compiler {
        Compiler {
            chunk: Chunk::new(),
            global: HashMap::new(),
        }
    }

    pub(crate) fn compile(&mut self, ast: Box<Stmt>) {
        #[cfg(feature = "compiler_dev")]
        println!("compile ast: {:#?}", ast);

        self.compile_stmt(ast);
        self.emit(OpCode::Return as u8);
    }

    pub(crate) fn pop_chunk(&mut self) -> Chunk {
        #[cfg(feature = "compiler_dev")]
        println!("compiled chunk: {:#?}", self.chunk);

        std::mem::replace(&mut self.chunk, Chunk::new())
    }

    fn compile_stmt(&mut self, stmt: Box<Stmt>) {
        match stmt.node {
            StmtKind::ExprStmt { expr } => {
                self.compile_expr(expr);
                self.emit(OpCode::Pop as u8)
            },
            StmtKind::VarDec { name, value } => self.compile_var_dec(name, value),
        }
    }

    fn compile_var_dec(&mut self, name: String, value: Box<Expr>) {
        self.compile_expr(value);
        // TODO: ensure len of global low than u16::MAX
        let i = self.global.len() as u16;
        self.global.insert(name, i);
        self.emit(OpCode::DefineGlobal as u8);
        // TODO: add long byte(u16) support
        self.emit(i as u8);
    }

    fn compile_expr(&mut self, expr: Box<Expr>) {
        match expr.node {
            ExprKind::Binary { left, op, right } => {
                self.compile_expr(left);
                self.compile_expr(right);
                self.emit_binary_op(op);
            }
            ExprKind::Group { body } => {
                self.compile_expr(body);
            }
            ExprKind::Literal { value } => {
                self.compile_literal(value);
            }
            ExprKind::Unary { op, operand } => {
                self.compile_expr(operand);
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
            ParseObj::Bool(b) => self.emit_constant(Value::Bool(b)),
            ParseObj::Int(v) => self.emit_constant(Value::Int(v as i64)),
            ParseObj::Float(v) => self.emit_constant(Value::Float(v)),
            ParseObj::Str(s) => self.emit_constant(Value::Str(s)),
            ParseObj::Ident(name) => {
                // TODO: add NameError to point name is not defined
                let i = self.global.get(&name).unwrap().clone();
                self.emit(OpCode::GetGlobal as u8);
                // TODO: long byte(u16) support
                self.emit(i as u8);
            },
        }
    }

    fn emit_unaryop(&mut self, op: Unaryop) {
        match op {
            Unaryop::Not => self.emit(OpCode::Not as u8),
            Unaryop::Minus => todo!(),
        }
    }

    fn emit_constant(&mut self, value: Value) {
        let index = self.chunk.write_constant(value);
        self.chunk.write_code(OpCode::Constant as u8);

        // TODO: add two byte argument support
        if index > u8::MAX as usize {
            todo!()
        }
        self.chunk.write_code(index as u8);
    }
}
