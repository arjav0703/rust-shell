mod autocomplete;
use autocomplete::ShellHelper;
use funcs::parse_args;
use rustyline::Editor;
pub mod builtin_functions;
pub mod ext_commands;
pub mod funcs;
pub mod history;

pub const BUILTINS: [&str; 7] = ["echo", "exit", "type", "pwd", "history", "clear", "cd"];

fn main() {
    run_loop();
}

fn run_loop() {
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
        let (cmd, arg) = parse_args(args);

        funcs::matcher_ext(arg, cmd, &BUILTINS, file_path);
    }
}
