use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("db > ");
        stdout.flush()?;

        input.clear();
        stdin.read_line(&mut input)?;
        if input.ends_with('\n') {
            input.pop();
        }

        if input.is_empty() || input == ".exit" {
            break;
        } else {
            println!("Unrecognized command {:?}.", input);
        }
    }

    Ok(())
}
