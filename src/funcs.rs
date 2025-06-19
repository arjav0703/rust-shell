use crate::builtin_functions;
use crate::ext_commands;

pub fn parse_redirection(input: &str) -> (String, Option<String>) {
    // Look for redirection operators

    if let Some(redirect_pos) = input.find(" > ") {
        let (cmd_part, file_part) = input.split_at(redirect_pos);

        let file_path = file_part.trim_start_matches(" > ").trim();

        return (cmd_part.to_string(), Some(file_path.to_string()));
    } else if let Some(redirect_pos) = input.find(" 1> ") {
        let (cmd_part, file_part) = input.split_at(redirect_pos);

        let file_path = file_part.trim_start_matches(" 1> ").trim();

        return (cmd_part.to_string(), Some(file_path.to_string()));
    }

    // No redirection found

    (input.to_string(), None)
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
