// builtin functions for the shell

use crate::ext_commands::find_executable;
use crate::funcs::write_to_file;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
//
pub fn echo(args: &[String], file_path: Option<String>) {
    if args.is_empty() {
    } else if let Some(path) = file_path {
        let content = args.join(" ").to_string();
        write_to_file(&path, &content);
    } else {
        println!("{} ", args.join(" "));
    }
}

pub fn get_type(name: &str, builtins: &[&str]) {
    if builtins.contains(&name) {
        println!("{} is a shell builtin", name);
    } else if let Some(path) = find_executable(name) {
        println!("{} is {}", name, path.display());
    } else {
        println!("{}: not found", name);
    }
}

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd: {}", e),
    }
}

pub fn cd(args: &[String]) {
    if args.is_empty() {
        eprintln!("cd: missing argument");
        return;
    }

    let target = &args[0];

    if target == "~" {
        if let Some(home) = env::var_os("HOME") {
            if env::set_current_dir(home).is_err() {
                eprintln!("cd: failed to change to home directory");
            }
        } else {
            eprintln!("cd: HOME not set");
        }
        return;
    }
    if env::set_current_dir(target).is_err() {
        eprintln!("cd: {}: No such file or directory", target);
    }
}

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

    pub fn show(&self) {
        let mut counter = 1;
        match fs::read_to_string(&self.history_file) {
            Ok(content) => {
                for line in content.lines() {
                    counter += 1;
                    println!("  {} {}", counter, line);
                }
            }
            Err(e) => eprintln!("Error reading history file: {}", e),
        }
    }

    pub fn clear(&self) {
        if fs::remove_file(&self.history_file).is_err() {
            eprintln!("Error clearing history file");
        }
    }
}
