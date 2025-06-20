mod autocomplete;
use autocomplete::ShellHelper;
use rustyline::Editor;
pub mod builtin_functions;
pub mod ext_commands;
pub mod funcs;
fn main() {
    run_loop();
}

fn run_loop() {
    let builtins = ["echo", "exit", "type", "pwd", "history", "clear", "cd"];

    let mut rl = Editor::new().unwrap();
    rl.set_helper(Some(ShellHelper));

    loop {
        let input = rl.readline("$ ").unwrap();

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let (args, file_path) = funcs::parse_redirection(input);
        //dbg!("Parsed input: {} {}", &args, &file_path);
        let (cmd, arg) = parse_cmd_and_args(args);

        funcs::matcher_ext(arg, cmd, &builtins, file_path);
    }
}

/// Splits the raw input line into the command name and a Vec of its arguments
fn parse_cmd_and_args(input: String) -> (String, Vec<String>) {
    let mut parts = shlex::split(&input).unwrap_or_default();

    let cmd = parts.first().cloned().unwrap_or_default();

    let args = if parts.len() > 1 {
        parts.split_off(1)
    } else {
        Vec::new()
    };

    (cmd, args)
}
