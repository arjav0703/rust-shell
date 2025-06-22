use crate::builtin_functions;
use crate::ext_commands;
use crate::history::History;

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

    (input.to_string(), None)
}

pub fn matcher_ext(
    args: Vec<String>,
    cmd: String,
    builtins: &[&str],
    file_path: Option<String>,
    history_file: &str,
) {
    let mut history = History::new(String::from(history_file));
    history.add(&cmd, &args);

    match cmd.as_str() {
        "exit" => {
            std::process::exit(0);
        }
        "echo" => builtin_functions::echo(&args, file_path),
        "type" => {
            let target = args.first().map(|s| s.as_str()).unwrap_or("");
            builtin_functions::get_type(target, builtins)
        }
        "pwd" => builtin_functions::pwd(),
        "cd" => builtin_functions::cd(&args),
        "history" => history.show_last(&args),
        "clear" => history.clear(),
        other => {
            ext_commands::execute_cmd(other, &args, file_path);
            //builtin_functions::redirect::run_with_redirection(other, &args, builtins);
        }
    }
}

pub fn write_to_file(file_path: &str, content: &str) {
    // Create parent dirs
    if let Some(parent) = std::path::Path::new(file_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let mut out = content.to_string();
    if !out.ends_with('\n') {
        out.push('\n');
    }

    match std::fs::write(file_path, out) {
        Ok(_) => {}
        Err(e) => eprintln!("Error writing to file: {}", e),
    }
}

/// Splits the raw input line into the command name and a Vec of its arguments
pub fn parse_args(input: String) -> (String, Vec<String>) {
    let mut parts = shlex::split(&input).unwrap_or_default();

    let cmd = parts.first().cloned().unwrap_or_default();

    let args = if parts.len() > 1 {
        parts.split_off(1)
    } else {
        Vec::new()
    };

    (cmd, args)
}
