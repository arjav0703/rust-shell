use crate::ext_commands::execute_cmd;
use std::fs::File;
use std::process::{Command, Stdio};

/// Executes a command with its arguments, optionally redirecting stdout
/// if the arguments contain a `>` token followed by a filename.
///
/// # Example
///
/// let cmd = "echo";
/// let args = vec!["hello".into(), ">".into(), "out.txt".into()];
/// let status = run_with_redirection(cmd, &args)?;
/// assert!(status.success());
pub fn run_with_redirection(cmd: &str, args: &[String]) {
    // Look for a ">" in args
    if let Some(pos) = args.iter().position(|s| s == ">") {
        // Ensure there is a file name after ">"
        if pos + 1 >= args.len() {
            panic!("No filename provided for redirection");
        }

        // Split into the real args and the output file
        let real_args = &args[..pos];
        let out_file = &args[pos + 1];

        // Open (or create/truncate) the output file
        let file = File::create(out_file)
            .unwrap_or_else(|e| panic!("Failed to create {}: {}", out_file, e));

        // Build and spawn the command, redirecting stdout
        let child = Command::new(cmd)
            .args(real_args)
            .stdout(Stdio::from(file))
            .spawn()
            .unwrap_or_else(|e| panic!("Failed to spawn {}: {}", cmd, e));

        std::mem::drop(child);
    } else {
        execute_cmd(cmd, args);
    }
}
