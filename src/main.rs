use std::{process::exit, io::Write};

use crate::frontend::parser::Parser;

mod frontend;
mod backend;

fn main() {
    let mut vm = backend::vm::VM::new();
    loop {    
        let mut data = String::new();
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut data).unwrap();
        if data == "exit\n" {
            exit(0);
        }
        let mut lexer = frontend::lexer::Lexer::new(data);
        let tokens = lexer.tokenize();
        let parser = Parser::new(tokens);
        vm.run(parser);
    }
}
