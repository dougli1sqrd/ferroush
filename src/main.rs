use std::io;
use std::io::Write;
use std::env;

mod command;
mod parse;

fn main() {
    println!("Hello, welcome to Ferroush!");
    repl();
}

/// Reads a line from stdin
fn read_line() -> String {
    let mut buffer = String::new();
    let read_result = io::stdin().read_line(&mut buffer);
    match read_result {
        Ok(_) => String::from(buffer.trim_right()),
        Err(error) => {
            println!("{}", error);
            String::new()
        }
    }
}

struct CommandAndArgs<'a> {
    command_name: String,
    arguments: Vec<& 'a str>
}

fn parse_line<'a>(line: & 'a str) -> CommandAndArgs<'a> {
    let splitline: Vec<&str> = line.split(' ').collect();
    let separated = &splitline.split_first().unwrap();
    CommandAndArgs {
        command_name: String::from(*separated.0),
        arguments: (*separated.1).to_vec()
    }
}

fn current_user() -> String {
    match env::var_os("USER") {
        Some(user) => format!("{:?}", user),
        None => String::from("unknown")
    }
}

fn current_dir() -> String {
    match env::current_dir() {
        Ok(current) => format!("{}", current.display()),
        Err(_) => {
            println!("Current working directory is no longer valid");
            String::from("???")
        }
    }
}

fn repl() {
    loop {
        print!("{} {} $ ", current_user(), current_dir());
        io::stdout().flush();
        let line = read_line();
        let parsed = parse_line(&line);
        if !parsed.command_name.is_empty() {
            command::run(parsed.command_name, parsed.arguments)
        }
    }
}
