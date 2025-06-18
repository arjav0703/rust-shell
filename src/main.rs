use std::io::{self, Write};
pub mod builtin_functions;
pub mod ext_commands;
pub mod funcs;
fn main() {
    run_loop();
}

fn run_loop() {
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

        funcs::matcher_redirect(args, cmd, &builtins);
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
