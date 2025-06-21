use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

pub struct History {
    history_file: String,
}

impl History {
    pub fn new(filename: String) -> Self {
        History {
            history_file: filename,
        }
    }

    /// Append a single command + args to the history file
    pub fn add(&self, cmd: &str, args: &[String]) {
        let line = format!("{} {}\n", cmd, args.join(" "));
        let result = OpenOptions::new()
            .create(true) // create file if it doesn’t exist
            .append(true) // open in append mode
            .open(&self.history_file)
            .and_then(|mut file| file.write_all(line.as_bytes()));

        if let Err(e) = result {
            eprintln!("Error writing to history file {}: {}", self.history_file, e);
        }
    }

    pub fn show_last(&self, args: &[String]) {
        let mut count = 100;
        let mut read_from = &self.history_file;

        // detect "-r" flag
        if args.len() >= 2 && args[0] == "-r" {
            read_from = &args[1];
            if let Some(_n) = args.get(2).and_then(|s| s.parse::<usize>().ok()) {
                return;
            }
            self.append_from_file(read_from);
            return;
        } else if args.len() >= 2 && args[0] == "-w" {
            if let Some(filename) = args.get(1) {
                self.write_to_file(filename);
            } else {
                eprintln!("Usage: history -w <filename>");
            }
            return;
        } else if args.len() >= 2 && args[0] == "-a" {
            if let Some(filename) = args.get(1) {
                self.appent_to_file(filename);
            } else {
                eprintln!("Usage: history -a <filename>");
            }
            return;
        } else if let Some(n) = args.first().and_then(|s| s.parse::<usize>().ok()) {
            count = n;
        }

        // read the (possibly updated) history file
        let content = match fs::read_to_string(&self.history_file) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to read {}: {}", &self.history_file, e);
                return;
            }
        };

        let lines: Vec<&str> = content.lines().collect();
        let total = lines.len();
        let start = if count >= total { 0 } else { total - count };

        for (i, &line) in lines[start..].iter().enumerate() {
            println!("{:5}  {}", start + i + 1, line);
        }
    }

    /// Wipe out the history file entirely
    pub fn clear(&self) {
        if let Err(e) = fs::remove_file(&self.history_file) {
            eprintln!("Error clearing history file {}: {}", &self.history_file, e);
        }
    }

    fn append_from_file(&self, filename: &str) {
        match fs::read_to_string(filename) {
            Ok(contents) => {
                let result = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.history_file)
                    .and_then(|mut f| f.write_all(contents.as_bytes()));
                if let Err(e) = result {
                    eprintln!(
                        "Error appending {} to {}: {}",
                        filename, self.history_file, e
                    );
                }
            }
            Err(e) => {
                eprintln!("Failed to read {}: {}", filename, e);
            }
        }
    }

    fn appent_to_file(&self, filename: &str) {
        match fs::read_to_string(filename) {
            Ok(contents) => {
                let result = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(filename)
                    .and_then(|mut f| f.write_all(contents.as_bytes()));
                if let Err(e) = result {
                    eprintln!(
                        "Error appending {} to {}: {}",
                        filename, self.history_file, e
                    );
                }
            }
            Err(e) => {
                eprintln!("Failed to read {}: {}", filename, e);
            }
        }
    }

    fn write_to_file(&self, filename: &str) {
        let content = match fs::read_to_string(&self.history_file) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to read {}: {}", &self.history_file, e);
                return;
            }
        };

        fs::write(filename, content).unwrap_or_else(|e| {
            eprintln!("Error writing to file {}: {}", filename, e);
        });
    }
}
