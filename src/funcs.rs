use crate::builtin_functions;
use crate::ext_commands;

pub fn matcher_redirect(args: Vec<String>, cmd: String, builtins: &[&str]) {
    match cmd.as_str() {
        "exit" => {
            std::process::exit(0);
        }
        "echo" => builtin_functions::echo(&args),
        "type" => {
            let target = args.first().map(|s| s.as_str()).unwrap_or("");
            builtin_functions::get_type(target, builtins)
        }
        "pwd" => builtin_functions::pwd(),
        "cd" => builtin_functions::cd(&args),
        other => {
            //ext_commands::execute_cmd(other, &args);
            builtin_functions::redirect::run_with_redirection(other, &args, builtins);
        }
    }
}
pub fn matcher_ext(args: Vec<String>, cmd: String, builtins: &[&str]) {
    match cmd.as_str() {
        "exit" => {
            std::process::exit(0);
        }
        "echo" => builtin_functions::echo(&args),
        "type" => {
            let target = args.first().map(|s| s.as_str()).unwrap_or("");
            builtin_functions::get_type(target, builtins)
        }
        "pwd" => builtin_functions::pwd(),
        "cd" => builtin_functions::cd(&args),
        other => {
            ext_commands::execute_cmd(other, &args);
            //builtin_functions::redirect::run_with_redirection(other, &args, builtins);
        }
    }
}
