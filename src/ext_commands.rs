use crate::funcs::write_to_file;
/// Look in the current directory and then each entry in $PATH for an executable named `cmd`.
use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn find_executable(cmd: &str) -> Option<std::path::PathBuf> {
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
//pub fn execute_cmd(cmd: &str, args: &[String], file_path: Option<String>) {
//    if find_executable(cmd).is_some() {
//        let mut child = match Command::new(cmd)
//            .args(args)
//            .stdin(Stdio::inherit())
//            .stdout(Stdio::inherit())
//            .stderr(Stdio::inherit())
//            .spawn()
//        {
//            Ok(child) => child,
//            Err(e) => {
//                eprintln!("{}: failed to execute: {}", cmd, e);
//                return;
//            }
//        };
//
//        // wait for it to finish
//        if let Err(e) = child.wait() {
//            eprintln!("{}: failed while waiting: {}", cmd, e);
//        }
//    } else {
//        eprintln!("{}: command not found", cmd);
//    }
//}
pub fn execute_cmd(cmd: &str, args: &[String], file_path: Option<String>) {
    // Quick check to see if the executable exists on $PATH
    if find_executable(cmd).is_none() {
        eprintln!("{}: command not found", cmd);
        return;
    }

    match file_path {
        // If the user asked for redirection, we capture stdout in-memory
        Some(path) => {
            // Run to completion, capturing stdout/stderr
            match Command::new(cmd)
                .args(args)
                .stdin(Stdio::inherit()) // still pipe through your terminal’s stdin
                .stderr(Stdio::inherit()) // errors still go to your terminal
                .output()
            {
                Ok(output) => {
                    if !output.status.success() {
                        eprintln!("{} exited with code {:?}", cmd, output.status.code());
                    }
                    write_to_file(&path, &String::from_utf8_lossy(&output.stdout));
                }
                Err(e) => {
                    eprintln!("{}: failed to execute: {}", cmd, e);
                }
            }
        }

        // No file redirection requested → inherit stdout/stderr
        None => {
            match Command::new(cmd)
                .args(args)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
            {
                Ok(mut child) => {
                    if let Err(e) = child.wait() {
                        eprintln!("{}: failed while waiting: {}", cmd, e);
                    }
                }
                Err(e) => {
                    eprintln!("{}: failed to execute: {}", cmd, e);
                }
            }
        }
    }
}
