mod ast;
mod compiler;
mod lexer;
mod location;
mod parser;
mod token;

use lexer::Cursor;
use parser::Parser;
use vm::chunk::Chunk;

pub struct Compiler {
    compiler: compiler::Compiler,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            compiler: compiler::Compiler::new(),
        }
    }

    pub fn compile(&mut self, raw_code: &str) -> Chunk {
        let cursor = Cursor::new(raw_code);
        let mut parser = Parser::new(cursor);
        let ast = parser.parse();
        self.compiler.compile(*ast);
        self.compiler.pop_chunk()
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
