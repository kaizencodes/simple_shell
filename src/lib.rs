use std::{
    env,
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

pub fn process_input(input: &str, prev_dir: &mut PathBuf) {
    let args: Vec<&str> = input.trim().split(" ").collect();
    match args[0] {
        "cd" => {
            cd(args, prev_dir);
        }
        "" => {}
        _ => run_command(args),
    };
}

fn cd(args: Vec<&str>, prev_dir: &mut PathBuf) {
    let curr_dir = env::current_dir().unwrap_or_default();
    if args.len() < 2 {
        let home_env = env::var("HOME").unwrap_or(String::from("/"));
        env::set_current_dir(&home_env).unwrap_or_else(|err| {
            eprintln!("Couldn't change directory to {home_env} due to {err}")
        });
    } else if args[1] == "-" {
        env::set_current_dir(&prev_dir).unwrap_or_else(|err| {
            eprintln!(
                "Couldn't change directory to {} due to {err}",
                prev_dir.display()
            )
        });
    } else {
        env::set_current_dir(args[1]).unwrap_or_else(|err| {
            eprintln!("Couldn't change directory to {} due to {err}", args[1])
        });
    }
    *prev_dir = curr_dir;
}

fn run_command(args: Vec<&str>) {
    let mut command = Command::new(args[0]);
    for arg in args[1..].iter() {
        command.arg(arg);
    }

    match command.output() {
        Ok(output) => io::stdout()
            .write_all(&output.stdout)
            .unwrap_or_else(|err| eprintln!("Couldn't write to the stdout: {err}")),
        Err(error) => {
            eprintln!("{error}");
        }
    }
}
