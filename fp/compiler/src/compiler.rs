use std::collections::HashMap;

use vm::{chunk::Chunk, op::OpCode, value::Value};

use crate::ast::{BinaryOp, Expr, ExprKind, ParseObj, Stmt, StmtKind, UnaryOp};

pub(crate) struct Compiler {
    chunk: Chunk,
    global: HashMap<String, u16>,
    compiling_var_dec: bool,
}

impl Compiler {
    pub(crate) fn new() -> Compiler {
        Compiler {
            chunk: Chunk::new(),
            global: HashMap::new(),
            compiling_var_dec: false,
        }
    }

    pub(crate) fn compile(&mut self, ast: Stmt) {
        #[cfg(feature = "compiler_dev")]
        println!("compile ast: {:#?}", ast);

        self.compile_stmt(ast);
        self.emit_opcode(OpCode::Return);
    }

    pub(crate) fn pop_chunk(&mut self) -> Chunk {
        #[cfg(feature = "compiler_dev")]
        println!("compiled chunk: {:#?}", self.chunk);

        std::mem::replace(&mut self.chunk, Chunk::new())
    }

    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt.node {
            StmtKind::ExprStmt { expr } => {
                self.compile_expr(*expr);
                self.emit_opcode(OpCode::Pop)
            }
            StmtKind::VarDec { name, value } => self.compile_var_dec(name, *value),
        }
    }

    fn compile_var_dec(&mut self, name: String, value: Expr) {
        self.compiling_var_dec = true;
        self.compile_expr(value);
        // TODO: ensure len of global low than u16::MAX
        let i = self.global.len() as u16;
        self.global.insert(name, i);

        if i > u8::MAX as u16 {
            self.emit_opcode(OpCode::DefineGlobalLong);
            // TODO: split u16 to 2 u8

            return;
        }

        self.emit(OpCode::DefineGlobal as u8);
        self.emit(i as u8);
        self.compiling_var_dec = false;
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
            ExprKind::Block { inner } => {
                if inner.is_empty() {
                    self.emit_opcode(OpCode::Nil);
                    return;
                }
                let mut inner = inner;
                // the inner isn't empty
                let end = inner.pop().unwrap();

                for stmt in inner {
                    self.compile_stmt(stmt);
                }

                if self.compiling_var_dec {
                    if let StmtKind::ExprStmt { expr } = end.node {
                        self.compile_expr(*expr);
                        return;
                    }
                }
                self.compile_stmt(end);
            }
        }
    }

    fn emit(&mut self, code: u8) {
        self.chunk.write_code(code);
    }

    fn emit_opcode(&mut self, code: OpCode) {
        self.chunk.write_code(code as u8);
    }

    fn emit_binary_op(&mut self, op: BinaryOp) {
        match op {
            BinaryOp::Add => self.emit_opcode(OpCode::Add),
            BinaryOp::Sub => self.emit_opcode(OpCode::Sub),
            BinaryOp::Mult => self.emit_opcode(OpCode::Mult),
            BinaryOp::Div => self.emit_opcode(OpCode::Div),
            BinaryOp::Eq => self.emit_opcode(OpCode::Eq),
            BinaryOp::NotEq => {
                self.emit(OpCode::Eq as u8);
                self.emit(OpCode::Not as u8);
            }
            BinaryOp::Gt => self.emit_opcode(OpCode::Gt),
            BinaryOp::GtE => todo!(),
            BinaryOp::Lt => self.emit_opcode(OpCode::Lt),
            BinaryOp::LtE => todo!(),
            BinaryOp::And => todo!(),
            BinaryOp::Or => todo!(),
        }
    }

    fn compile_literal(&mut self, value: ParseObj) {
        match value {
            ParseObj::Nil => self.emit_opcode(OpCode::Nil),
            ParseObj::Bool(b) => self.emit_constant(Value::Bool(b)),
            ParseObj::Int(v) => self.emit_constant(Value::Int(v as i64)),
            ParseObj::Float(v) => self.emit_constant(Value::Float(v)),
            ParseObj::Str(s) => self.emit_constant(Value::Str(s)),
            ParseObj::Ident(name) => {
                // TODO: add NameError to point name is not defined
                let i = *self.global.get(&name).unwrap();
                self.emit_opcode(OpCode::GetGlobal);
                // TODO: long byte(u16) support
                self.emit(i as u8);
            }
        }
    }

    fn emit_unaryop(&mut self, op: UnaryOp) {
        match op {
            UnaryOp::Not => self.emit_opcode(OpCode::Not),
            UnaryOp::Minus => todo!(),
        }
    }

    fn emit_constant(&mut self, value: Value) {
        let index = self.chunk.write_constant(value);
        self.emit_opcode(OpCode::Constant);

        // TODO: add two byte argument support
        if index > u8::MAX as usize {
            todo!()
        }
        self.emit(index as u8);
    }
}
