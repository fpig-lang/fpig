use std::io;

use compiler::{cursor::Cursor, parser::Parser};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let cursor = Cursor::new(&input);
    let mut parser = Parser::new(cursor);
    println!("{:?}", parser.parse());
}
