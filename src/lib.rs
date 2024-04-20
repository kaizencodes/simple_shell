use std::{
    env,
    io::{self, Write},
    process::Command,
};

pub fn process_input(input: &str) {
    let args: Vec<&str> = input.trim().split(" ").collect();
    match args[0] {
        "cd" => {
            cd(args);
        }
        "" => {}
        _ => run_command(args),
    };
}

fn cd(args: Vec<&str>) {
    let path: &str;
    let home_env = env::var("HOME").unwrap();

    if args.len() < 2 {
        path = &home_env;
    } else {
        path = args[1];
    }
    env::set_current_dir(path).unwrap();
}

fn run_command(args: Vec<&str>) {
    let mut command = Command::new(args[0]);
    for arg in args[1..].iter() {
        command.arg(arg);
    }

    match command.output() {
        Ok(output) => {
            io::stdout().write_all(&output.stdout).unwrap();
        }
        Err(error) => {
            eprintln!("{error}");
        }
    }
}
