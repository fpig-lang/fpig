use std::io::{self, Write};

use vm::vm::Vm;

fn main() {
    loop {
        print!("fpig> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let chunk = compiler::compile(&input);
        let mut vm = Vm::new(chunk);
        println!("{:#?}", vm.interpret());
    }
}
