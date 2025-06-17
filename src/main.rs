use std::io::{self, Write};
mod builtin_functions;
pub mod ext_commands;

fn main() {
    let builtins = ["echo", "exit", "type", "pwd"];

    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            continue;
        }
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let (cmd, args) = parse_cmd_and_args(input);

        match cmd.as_str() {
            "exit" => break,
            "echo" => builtin_functions::echo(&args),
            "type" => {
                let target = args.first().map(|s| s.as_str()).unwrap_or("");
                builtin_functions::get_type(target, &builtins)
            }
            "pwd" => builtin_functions::pwd(),
            "cd" => builtin_functions::cd(&args),
            other => {
                ext_commands::execute_cmd(other, &args);
            }
        }
    }
}

/// Splits the raw input line into the command name and a Vec of its arguments
fn parse_cmd_and_args(input: &str) -> (String, Vec<String>) {
    let mut parts = shlex::split(input).unwrap_or_default();

    let cmd = parts.first().cloned().unwrap_or_default();

    let args = if parts.len() > 1 {
        parts.split_off(1)
    } else {
        Vec::new()
    };

    (cmd, args)
}
