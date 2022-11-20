use std::io::{self, Write};

use compiler::Compiler;
use vm::vm::Vm;

fn main() {
    let mut compiler = Compiler::new();
    let mut vm = Vm::new();
    loop {
        print!("fpig> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let chunk = compiler.compile(&input);
        vm.interpret(chunk);
    }
}
