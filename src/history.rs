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

    pub fn add(&self, cmd: &str, args: &[String]) {
        let line = format!("{} {}\n", cmd, args.join(" "));
        let result = OpenOptions::new()
            .create(true) // create file if it doesn’t exist
            .append(true) // open in append mode
            .open(&self.history_file)
            .and_then(|mut file| file.write_all(line.as_bytes()));

        if let Err(e) = result {
            eprintln!("Error writing to history file: {}", e);
        }
    }

    //pub fn show(&self, number: usize) {
    //    let mut counter = 1;
    //    match fs::read_to_string(&self.history_file) {
    //        Ok(content) => {
    //            for line in content.lines() {
    //                counter += 1;
    //                println!("  {} {}", counter, line);
    //            }
    //        }
    //        Err(e) => eprintln!("Error reading history file: {}", e),
    //    }
    //}

    pub fn show_last(&self, args: &[String]) {
        let count = args
            .first()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(100);

        let content = match fs::read_to_string(&self.history_file) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to read {:?}: {}", &self.history_file, e);
                return;
            }
        };

        let lines: Vec<&str> = content.lines().collect();
        let total = lines.len();
        let start = if count >= total { 0 } else { total - count };

        for (idx, &line) in lines[start..].iter().enumerate() {
            println!("    {}  {}", start + idx + 1, line);
        }
    }

    pub fn clear(&self) {
        if fs::remove_file(&self.history_file).is_err() {
            eprintln!("Error clearing history file");
        }
    }
}
