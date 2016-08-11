mod builtins;

use std::io;
use std::io::Write;
use std::process;


pub struct ProcessResult {
    error_message: String,
    exit_status: u32
}

impl ProcessResult {
    fn init(error: String, exit_status: u32) -> ProcessResult {
        ProcessResult {
            error_message: error,
            exit_status: exit_status
        }
    }
}

pub fn run(name: String, args: Vec<&str>) {
    let cmd_result = run_command(name, &args);
    match cmd_result {
        Ok(_) => { },
        Err(proc_res) => {
            let mut err = io::stderr();
            write!(err, "{}\n", proc_res.error_message);
            err.flush().unwrap();
        }
    }
}

fn run_command(name: String, args: &[&str]) -> Result<(), ProcessResult> {
    let builtin_result = builtins::run_builtin(&name, args);
    match builtin_result {
        Ok(()) => Ok(()),
        Err(_) => run_binary(name, args)
    }
}

fn run_binary(name: String, args: &[&str]) -> Result<(), ProcessResult> {
    let mut command = process::Command::new(&name);
    command.args(args);
    let mut child = try!(command.spawn().map_err(|e| ProcessResult::init(format!("{}", e), 1)));
    match child.wait() {
        Ok(_) => Ok(()),
        Err(std_err) => Err(ProcessResult::init(std_err.to_string(), 1))
    }
}
