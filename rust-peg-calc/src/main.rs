#![feature(plugin)]
#![plugin(peg_syntax_ext)]

use std::io;
use std::io::Write;

mod ast;
mod eval;

peg_file! grammar("grammar.rustpeg");

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().ok().expect("Failed to flush prompt");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        if input.len() == 0 {
            break;
        }

        match grammar::expr(input) {
            Ok(ref parsed) => println!("{}", eval::evaluate(parsed)),
            Err(error) => println!("{:?}", error),
        };
    }
}
