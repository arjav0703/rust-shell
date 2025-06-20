// builtin functions for the shell

use crate::ext_commands::find_executable;
use crate::funcs::write_to_file;
use std::env;

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
        cd_home();
        return;
    }

    let target = &args[0];

    if target == "~" {
        cd_home();
        return;
    }
    if env::set_current_dir(target).is_err() {
        eprintln!("cd: {}: No such file or directory", target);
    }
}

fn cd_home() {
    if let Some(home) = env::var_os("HOME") {
        if env::set_current_dir(home).is_err() {
            eprintln!("cd: failed to change to home directory");
        }
    } else {
        eprintln!("cd: HOME not set");
    }
}
