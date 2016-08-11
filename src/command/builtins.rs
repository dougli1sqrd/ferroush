use std::path;
use std::env;
use std::io;

use command::ProcessResult;

fn find_builtin(builtin_name: &str) -> Option<fn(&[&str]) -> Result<(), ProcessResult>> {
    match builtin_name {
        "cd" => Some(cd),
        _ => None
    }
}

pub fn run_builtin(builtin_name: &str, args: &[&str]) -> Result<(), ProcessResult> {
    match find_builtin(builtin_name) {
        Some(func) => func(args),
        None => Err(ProcessResult::init(format!("Command '{}' not found", builtin_name), 1))
    }
}

fn cd(args: &[&str]) -> Result<(), ProcessResult> {

    let path = if args.is_empty() {
        None
    } else {
        Some(String::from(args[0]))
    };

    let cd_path = build_cd_path(path).unwrap();
    match env::set_current_dir(cd_path) {
        Ok(()) => Ok(()),
        Err(error) => Err(ProcessResult::init(format!("{}", error), 2))
    }
}

fn build_cd_path(path: Option<String>) -> Result<path::PathBuf, String> {
    match path {
        Some(p) => Ok(path::PathBuf::from(p)),
        None => env::home_dir().ok_or(String::from("Unable to find home directory"))
    }
}
