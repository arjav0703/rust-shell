use crate::ShellHelper;
use rustyline::Editor;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

//pub struct History {
//    history_file: String,
//    buffer: Vec<String>,
//    session_start_len: usize,
//}
//
//impl History {
//    /// Load existing history into memory
//    pub fn new(filename: String) -> Self {
//        let contents = fs::read_to_string(&filename).unwrap_or_default();
//        let buffer = contents.lines().map(|s| s.to_string()).collect::<Vec<_>>();
//        let session_start_len = buffer.len();
//
//        History {
//            history_file: filename,
//            buffer,
//            session_start_len,
//        }
//    }
//
//    pub fn add(&self, cmd: &str, args: &[String]) {
//        let line = if args.is_empty() {
//            format!("{}\n", cmd)
//        } else {
//            format!("{} {}\n", cmd, args.join(" "))
//        };
//
//        let result = OpenOptions::new()
//            .create(true)
//            .append(true)
//            .open(&self.history_file)
//            .and_then(|mut file| file.write_all(line.as_bytes()));
//
//        if let Err(e) = result {
//            eprintln!("Error writing to history file {}: {}", self.history_file, e);
//        }
//    }
//
//    pub fn show_last(&self, args: &[String]) {
//        let mut count = 100;
//        let mut read_from = &self.history_file;
//
//        // detect "-r" flag
//        if args.len() >= 2 && args[0] == "-r" {
//            read_from = &args[1];
//            if let Some(_n) = args.get(2).and_then(|s| s.parse::<usize>().ok()) {
//                return;
//            }
//            self.append_from_file(read_from);
//            return;
//        } else if args.len() >= 2 && args[0] == "-w" {
//            if let Some(filename) = args.get(1) {
//                self.write_to_file(filename);
//            } else {
//                eprintln!("Usage: history -w <filename>");
//            }
//            return;
//        } else if args.len() >= 2 && args[0] == "-a" {
//            if let Some(filename) = args.get(1) {
//                self.append_to_file(filename);
//            } else {
//                eprintln!("Usage: history -a <filename>");
//            }
//            return;
//        } else if let Some(n) = args.first().and_then(|s| s.parse::<usize>().ok()) {
//            count = n;
//        }
//
//        // read the (possibly updated) history file
//        let content = match fs::read_to_string(&self.history_file) {
//            Ok(s) => s,
//            Err(e) => {
//                eprintln!("Failed to read {}: {}", &self.history_file, e);
//                return;
//            }
//        };
//
//        let lines: Vec<&str> = content.lines().collect();
//        let total = lines.len();
//        let start = if count >= total { 0 } else { total - count };
//
//        for (i, &line) in lines[start..].iter().enumerate() {
//            println!("{:5}  {}", start + i + 1, line);
//        }
//    }
//
//    /// Wipe out the history file entirely
//    pub fn clear(&self) {
//        if let Err(e) = fs::remove_file(&self.history_file) {
//            eprintln!("Error clearing history file {}: {}", &self.history_file, e);
//        }
//    }
//
//    fn append_from_file(&self, filename: &str) {
//        match fs::read_to_string(filename) {
//            Ok(contents) => {
//                let result = OpenOptions::new()
//                    .create(true)
//                    .append(true)
//                    .open(&self.history_file)
//                    .and_then(|mut f| f.write_all(contents.as_bytes()));
//                if let Err(e) = result {
//                    eprintln!(
//                        "Error appending {} to {}: {}",
//                        filename, self.history_file, e
//                    );
//                }
//            }
//            Err(e) => {
//                eprintln!("Failed to read {}: {}", filename, e);
//            }
//        }
//    }
//
//    pub fn append_to_file(&self, target: &str) {
//        if self.buffer.len() <= self.session_start_len {
//            return;
//        }
//        let mut file = match OpenOptions::new().create(true).append(true).open(target) {
//            Ok(f) => f,
//            Err(e) => {
//                eprintln!("Error opening {}: {}", target, e);
//                return;
//            }
//        };
//
//        for line in &self.buffer[self.session_start_len..] {
//            if let Err(e) = writeln!(file, "{}", line) {
//                eprintln!("Error writing to {}: {}", target, e);
//                return;
//            }
//        }
//    }
//
//    fn write_to_file(&self, filename: &str) {
//        let content = match fs::read_to_string(&self.history_file) {
//            Ok(s) => s,
//            Err(e) => {
//                eprintln!("Failed to read {}: {}", &self.history_file, e);
//                return;
//            }
//        };
//
//        fs::write(filename, content).unwrap_or_else(|e| {
//            eprintln!("Error writing to file {}: {}", filename, e);
//        });
//    }
//}

pub fn history_handler(
    rl: &mut Editor<ShellHelper, rustyline::history::DefaultHistory>,
    file_path: Option<String>,
    args: &[String],
) {
    let mut count = 100;
    let def_path = ".shell_default_history";
    rl.save_history(def_path)
        .unwrap_or_else(|e| eprintln!("Error saving history: {}", e));
    strip_version_header(def_path).unwrap_or_else(|e| eprintln!("strip header: {}", e));

    // detect "-r" flag
    if args.len() >= 2 && args[0] == "-r" {
        fs::read_to_string(&args[1])
            .map(|content| {
                let lines: Vec<&str> = content.lines().collect();
                for (i, line) in lines.iter().enumerate() {
                    println!("{:5}  {}", i + 1, line);
                }
            })
            .unwrap_or_else(|e| eprintln!("Failed to read {}: {}", args[1], e));
    } else if args.len() >= 2 && args[0] == "-w" {
        rl.save_history(&args[1])
            .unwrap_or_else(|e| eprintln!("Error saving history to {}: {}", args[1], e));

        return;
    } else if args.len() >= 2 && args[0] == "-a" {
        rl.append_history(args[1].as_str())
            .unwrap_or_else(|e| eprintln!("Error appending history to {}: {}", args[1], e));
        return;
    } else if let Some(n) = args.first().and_then(|s| s.parse::<usize>().ok()) {
        count = n;
    }

    let content = match fs::read_to_string(def_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read {}: {}", def_path, e);
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

fn strip_version_header(path: &str) -> io::Result<()> {
    let data = fs::read_to_string(path)?;
    let mut lines = data.lines();
    lines.next();
    let mut f = fs::File::create(path)?;
    for line in lines {
        writeln!(f, "{}", line)?;
    }
    Ok(())
}
