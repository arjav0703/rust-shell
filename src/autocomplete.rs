use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{Context, Helper};
use std::fs;
use std::path::Path;

pub struct ShellHelper;

//impl Completer for MyHelper {
//    type Candidate = Pair;
//
//    fn complete(
//        &self,
//        line: &str,
//        pos: usize,
//        _ctx: &Context<'_>,
//    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
//        let commands = ["echo ", "exit "];
//        // find starting position of the word
//        let start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
//        let word = &line[start..pos];
//        // filter the commands based on the input word
//        let pairs: Vec<Pair> = commands
//            .iter()
//            .filter(|&&cmd| cmd.starts_with(word))
//            .map(|&cmd| Pair {
//                display: cmd.to_string(),
//                replacement: cmd.to_string(),
//            })
//            .collect();
//        Ok((start, pairs))
//    }
//}

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let mut candidates = Vec::new();

        // split input up to cursor position
        let (before_cursor, _) = line.split_at(pos);
        let tokens: Vec<&str> = before_cursor.split_whitespace().collect();
        let last_token = tokens.last().unwrap_or(&"");

        if tokens.len() <= 1 && pos <= before_cursor.trim_end().len() {
            let builtins = vec!["echo", "exit", "type", "pwd", "cd"];
            for cmd in builtins {
                if cmd.starts_with(last_token) {
                    candidates.push(Pair {
                        display: format!("{} ", cmd),
                        replacement: format!("{} ", cmd),
                    });
                }
            }

            // executables from $PATH
            let path_env = std::env::var("PATH").unwrap_or_default();
            for dir in path_env.split(':') {
                if let Ok(entries) = fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        if let Ok(file_name) = entry.file_name().into_string() {
                            if file_name.starts_with(last_token)
                                && Path::new(&entry.path()).is_file()
                            {
                                candidates.push(Pair {
                                    display: format!("{} ", file_name),
                                    replacement: format!("{} ", file_name),
                                });
                            }
                        }
                    }
                }
            }
        } else {
            let dir = if last_token.contains('/') {
                // Extract directory path from last token
                Path::new(last_token)
                    .parent()
                    .unwrap_or_else(|| Path::new("."))
            } else {
                Path::new(".")
            };
            let prefix = Path::new(last_token)
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or(last_token);

            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        if file_name.starts_with(prefix) {
                            let path = entry.path();
                            let display = file_name;
                            let replacement = if path.is_dir() {
                                format!("{}/", path.display())
                            } else {
                                path.display().to_string()
                            };
                            candidates.push(Pair {
                                display,
                                replacement,
                            });
                        }
                    }
                }
            }
        }

        Ok((0, candidates))
    }
}

impl Hinter for ShellHelper {
    type Hint = String;
}

impl Highlighter for ShellHelper {}
impl Validator for ShellHelper {
    fn validate(&self, _ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        Ok(ValidationResult::Valid(None))
    }
}
impl Helper for ShellHelper {}
