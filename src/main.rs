use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    let builtins = ["echo", "exit", "type"];

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
            "echo" => builtin_echo(&args),
            "type" => {
                let target = args.get(0).map(|s| s.as_str()).unwrap_or("");
                builtin_type(target, &builtins)
            }
            other => {
                execute_external(other, &args);
            }
        }
    }
}

/// Splits the raw input line into the command name and a Vec of its arguments.
fn parse_cmd_and_args(input: &str) -> (String, Vec<String>) {
    let parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    let cmd = parts.get(0).cloned().unwrap_or_default();
    let args = if parts.len() > 1 {
        parts[1..].to_vec()
    } else {
        Vec::new()
    };

    (cmd, args)
}

fn builtin_echo(args: &[String]) {
    if args.is_empty() {
        println!();
    } else {
        println!("{}", args.join(" "));
    }
}

fn builtin_type(name: &str, builtins: &[&str]) {
    if builtins.contains(&name) {
        println!("{} is a shell builtin", name);
    } else if let Some(path) = find_executable(name) {
        println!("{} is {}", name, path.display());
    } else {
        println!("{}: not found", name);
    }
}

/// Look in the current directory and then each entry in $PATH for an executable named `cmd`.
fn find_executable(cmd: &str) -> Option<std::path::PathBuf> {
    let cwd = env::current_dir().ok()?;
    let local = cwd.join(cmd);
    if is_executable(&local) {
        return Some(local);
    }

    // check each entry in PATH
    if let Ok(path_var) = env::var("PATH") {
        for dir in path_var.split(':') {
            let candidate = Path::new(dir).join(cmd);
            if is_executable(&candidate) {
                return Some(candidate);
            }
        }
    }

    None
}

fn is_executable(p: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    p.is_file()
        && p.metadata()
            .map(|m| m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
}

/// Execute an external command (non-builtin) by spawning it and inheriting stdout/stderr.
fn execute_external(cmd: &str, args: &[String]) {
    if find_executable(cmd).is_some() {
        let mut child = match Command::new(cmd)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                eprintln!("{}: failed to execute: {}", cmd, e);
                return;
            }
        };

        // wait for it to finish
        if let Err(e) = child.wait() {
            eprintln!("{}: failed while waiting: {}", cmd, e);
        }
    } else {
        eprintln!("{}: command not found", cmd);
    }
}
