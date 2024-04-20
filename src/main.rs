use std::{
    env,
    io::{self, Write},
};

use simple_shell::process_input;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        print!("{} > ", env::current_dir().unwrap().display());
        io::stdout().flush().expect("Could not flush stdout");
        match stdin.read_line(&mut buffer) {
            Ok(_) => {
                process_input(&buffer);
                buffer.clear();
            }
            Err(error) => eprintln!("Couldn't read from input: {error}"),
        }
    }
}
