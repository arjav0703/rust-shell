use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    let builtins = ["echo", "exit", "type"];

    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let (command, args) = get_cmd_args(input);

        match command.as_str() {
            "exit" => break,
            "echo" => echo_builtin(args),
            "type" => type_builtin(args.first().map_or("", String::as_str), &builtins),
            command => execute_external(command, args),
        }
    }
}

fn get_cmd_args(input: &str) -> (String, Vec<String>) {
    let input_map = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let command = input_map.get(0).cloned().unwrap_or_default();
    let args = input_map.into_iter().skip(1).collect::<Vec<String>>();

    (command, args)
}

fn echo_builtin(args: Vec<String>) {
    if args.is_empty() {
        println!();
    } else {
        println!("{}", args.join(" "));
    }
}

fn type_builtin(arg: &str, builtins: &[&str]) {
    if builtins.contains(&arg) {
        println!("{} is a shell builtin", arg);
    } else if let Some(full_path) = get_full_path(arg) {
        println!("{} is {}", arg, full_path);
    } else {
        println!("{}: not found", arg);
    }
}

fn get_full_path(command: &str) -> Option<String> {
    let current_dir = env::current_dir().ok()?;
    let local = current_dir.join(command);

    if local.exists() {
        return Some(local.to_string_lossy().into_owned());
    }

    if let Ok(path_var) = env::var("PATH") {
        for dir in path_var.split(':') {
            let candidate = Path::new(dir).join(command);
            if candidate.exists() {
                return Some(candidate.to_string_lossy().into_owned());
            }
        }
    }
    None
}

fn execute_external(command: &str, args: Vec<String>) {
    if let Some(full_path) = get_full_path(command) {
        let mut cmd = Command::new(full_path);

        cmd.args(args);

        match cmd.status() {
            Ok(status) => {
                if !status.success() {
                    eprintln!("{}: command failed with status {}", command, status);
                }
            }
            Err(e) => eprintln!("{}: failed to execute command: {}", command, e),
        }
    } else {
        println!("{}: command not found", command);
    }
}
