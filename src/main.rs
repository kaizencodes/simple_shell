use std::{
    env,
    io::{self, Write},
};

use simple_shell::process_input;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut prev_dir = env::current_dir().unwrap_or_default();
    loop {
        print!("{} > ", env::current_dir().unwrap_or_default().display());
        io::stdout()
            .flush()
            .unwrap_or_else(|err| eprintln!("Could not flush stdout: {err}"));
        match stdin.read_line(&mut buffer) {
            Ok(_) => {
                process_input(&buffer, &mut prev_dir);
                buffer.clear();
            }
            Err(error) => eprintln!("Couldn't read from input: {error}"),
        }
    }
}
