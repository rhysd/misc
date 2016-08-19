use std::io;
use std::io::Write;

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

        println!("TODO: Parse input and calculate/print the result\n");
    }
}
