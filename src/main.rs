use compiler::lexer::tokenize;

fn main() {
    let input = "let a = 1";
    let tokens = tokenize(input);
    for token in tokens {
        println!("{}", token);
    }
}
