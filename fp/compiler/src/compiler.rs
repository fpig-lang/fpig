use std::collections::HashMap;

use vm::{chunk::Chunk, op::OpCode, value::Value};

use crate::ast::{BinaryOp, Expr, ExprKind, ParseObj, Stmt, StmtKind, UnaryOp};

pub(crate) struct Compiler {
    chunk: Chunk,
    global: HashMap<String, u16>,
    scope: Vec<HashMap<String, u16>>,
    scope_depth: usize,
    stack_top: u16,
}

impl Compiler {
    pub(crate) fn new() -> Compiler {
        Compiler {
            chunk: Chunk::new(),
            global: HashMap::new(),
            scope: Vec::new(),
            scope_depth: 0,
            stack_top: 0,
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
        println!("compiled chunk: {:#04x?}", self.chunk);

        std::mem::replace(&mut self.chunk, Chunk::new())
    }

    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt.node {
            StmtKind::ExprStmt { expr } => {
                self.compile_expr(*expr);
                self.emit_opcode(OpCode::Pop)
            }
            StmtKind::VarDec { name, value } => self.compile_var_dec(name, *value),
            StmtKind::While { test, body } => self.compile_while(test, body),
        }
    }

    fn compile_var_dec(&mut self, name: String, value: Expr) {
        self.compile_expr(value);

        if self.scope_depth == 0 {
            // TODO: ensure len of global low than u16::MAX
            if self.global.len() > u16::MAX as usize {
                todo!()
            }
            let i = self.global.len() as u16;
            self.global.insert(name, i);

            if i > u8::MAX as u16 {
                self.emit_opcode(OpCode::SetGlobalL);
                self.emit_long_byte(i);
                return;
            }

            self.emit(OpCode::SetGlobal as u8);
            self.emit(i as u8);
            return;
        }

        // TODO: check the same variable name
        self.add_local(name);
    }

    fn compile_while(&mut self, test: Box<Expr>, body: Vec<Stmt>) {
        todo!()
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
                self.emit_unary_op(op);
            }
            ExprKind::Block { inner } => {
                self.compile_block(inner);
            }
            ExprKind::If { test, body, orelse } => {
                self.compile_if(test, body, orelse);
            }
        }
    }

    // it will generate:
    // { other }
    // { test }             <- an expr
    // +--- JumpIfFalse     <- will pop and check the value in the top of stack
    // |    { body }        <- a block expr
    // |    Jump -------+
    // +--> { else }    |   <- a block expr
    //      { other } <-+
    // the 'if' should be treated as an expr, that means `let a = if false { 1 } else { 2 }` is fine.
    // when there isn't a left value, 'if' will be treated as a ExprStmt, it will drop the value.
    fn compile_if(&mut self, test: Box<Expr>, body: Vec<Stmt>, orelse: Vec<Stmt>) {
        // TODO: this funcation is really a piece of shit.
        self.compile_expr(*test);
        self.emit_opcode(OpCode::JumpIfFalse);
        self.emit_long_byte(0);
        let now = self.chunk.get_code_len();
        self.compile_block(body);
        let end = self.chunk.get_code_len();
        if end - now + 3 > u16::MAX as usize {
            todo!()
        }
        if orelse.is_empty() {
            // no else
            self.emit_backfill_long(now - 2, (end - now) as u16);
            return;
        }
        self.emit_backfill_long(now - 2, (end - now) as u16 + 3);

        self.emit_opcode(OpCode::Jump);
        self.emit_long_byte(0);
        let now = self.chunk.get_code_len();
        self.compile_block(orelse);
        let end = self.chunk.get_code_len();
        if end - now + 3 > u16::MAX as usize {
            todo!()
        }
        self.emit_backfill_long(now - 2, (end - now) as u16);
    }

    // it will generate:
    // 1. Nil             when block is empty.
    // 2. { body }
    //    Nil             when last Stmt in block isn't ExprStmt
    // 3. { body }
    //    { last expr }   when last Stmt is a ExprStmt, it will be treated as an expr
    //    BlockEnd        <- always have this one, it will shift all the local variable in block and keep the return value.
    //    N               <- determine how many local variable should be shift.
    // block is an expr, it always return a value.
    // when there isn't a left value, the block will be treated as a ExprStmt and drop the return value.
    fn compile_block(&mut self, inner: Vec<Stmt>) {
        self.begin_scope();
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

        // make sure the end stmt will produce a value in stack
        if let StmtKind::ExprStmt { expr } = end.node {
            self.compile_expr(*expr);
        } else {
            self.compile_stmt(end);
            self.emit_opcode(OpCode::Nil);
        }

        self.end_scope();
    }

    fn compile_literal(&mut self, value: ParseObj) {
        match value {
            ParseObj::Nil => self.emit_opcode(OpCode::Nil),
            ParseObj::Bool(b) => self.emit_constant(Value::Bool(b)),
            ParseObj::Int(v) => self.emit_constant(Value::Int(v as i64)),
            ParseObj::Float(v) => self.emit_constant(Value::Float(v)),
            ParseObj::Str(s) => self.emit_constant(Value::Str(s)),
            ParseObj::Ident(name) => self.compile_variable(name),
        }
    }

    fn compile_variable(&mut self, name: String) {
        if self.scope_depth == 0 {
            if let Some(i) = self.global.get(&name).copied() {
                if i > u8::MAX as u16 {
                    self.emit_opcode(OpCode::GetGlobalL);
                    self.emit_long_byte(i);
                } else {
                    self.emit_opcode(OpCode::GetGlobal);
                    self.emit(i as u8);
                }
            } else {
                todo!()
            }
            return;
        }

        if let Some(i) = self.scope[self.scope_depth - 1].get(&name).copied() {
            if i > u8::MAX as u16 {
                self.emit_opcode(OpCode::GetLocalL);
                self.emit_long_byte(i);
            } else {
                self.emit_opcode(OpCode::GetLocal);
                self.emit(i as u8);
            }
        } else {
            todo!()
        }
    }

    // scope
    fn begin_scope(&mut self) {
        self.scope_depth += 1;
        self.scope.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        // TODO: check the size of 'count'
        let count = self.scope.last().unwrap().len() as u8;
        if count == 0 {
            return;
        }
        self.scope.pop();
        self.emit_opcode(OpCode::BlockEnd);
        self.emit(count);
        self.stack_top -= count as u16;
    }

    fn add_local(&mut self, name: String) {
        self.scope[self.scope_depth - 1].insert(name, self.stack_top);
        if self.stack_top == u16::MAX {
            todo!()
        }
        self.stack_top += 1;
    }

    // emit family
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

    fn emit_unary_op(&mut self, op: UnaryOp) {
        match op {
            UnaryOp::Not => self.emit_opcode(OpCode::Not),
            UnaryOp::Neg => todo!(),
        }
    }

    fn emit_constant(&mut self, value: Value) {
        let index = self.chunk.write_constant(value);
        self.emit_opcode(OpCode::Constant);
        if index > u16::MAX as usize {
            todo!()
        } else if index > u8::MAX as usize {
            self.emit_long_byte(index as u16);
        } else {
            self.emit(index as u8);
        }
    }

    fn emit_long_byte(&mut self, b: u16) {
        let bytes = b.to_be_bytes();
        self.emit(bytes[0]);
        self.emit(bytes[1]);
    }

    fn emit_backfill_long(&mut self, ip: usize, b: u16) {
        let bytes = b.to_be_bytes();
        self.chunk.backfill(ip, bytes[0]);
        self.chunk.backfill(ip + 1, bytes[1]);
    }
}
