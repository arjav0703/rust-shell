use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(long, default_value = ".shell_history")]
    history_file: String,
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }

    pub fn get_history_file(&self) -> &str {
        &self.history_file
    }
}
