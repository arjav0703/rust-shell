use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{Context, Helper};

pub struct MyHelper;

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let commands = ["echo ", "exit "];
        // find starting position of the word
        let start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let word = &line[start..pos];
        // filter the commands based on the input word
        let pairs: Vec<Pair> = commands
            .iter()
            .filter(|&&cmd| cmd.starts_with(word))
            .map(|&cmd| Pair {
                display: cmd.to_string(),
                replacement: cmd.to_string(),
            })
            .collect();
        Ok((start, pairs))
    }
}

// Minimal implementations for other helpers traits
impl Hinter for MyHelper {
    type Hint = String;
}

impl Highlighter for MyHelper {}
impl Validator for MyHelper {
    fn validate(&self, _ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        Ok(ValidationResult::Valid(None))
    }
}
impl Helper for MyHelper {}
