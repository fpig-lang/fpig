mod ast;
pub mod compiler;
pub mod cursor;
pub mod lexer;
pub mod parser;
mod token;

use compiler::Compiler;
use cursor::Cursor;
use parser::Parser;
use vm::chunk::Chunk;

pub fn compile(raw_code: &str) -> Chunk {
    let cursor = Cursor::new(raw_code);
    let mut parser = Parser::new(cursor);
    let ast = parser.parse();
    let mut compiler = Compiler::new();
    compiler.compile(*ast);
    compiler.pop_chunk()
}
