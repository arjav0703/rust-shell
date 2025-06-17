use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let (command, args) = eval(input);

        match command.as_str() {
            "exit" => break,
            "echo" => echo(args),
            "type" => type_fn(args.first().map_or("", String::as_str)),
            command => println!("{}: command not found", command),
        }
    }
}

fn eval(input: &str) -> (String, Vec<String>) {
    let input_map = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let command = input_map.get(0).cloned().unwrap_or_default();
    let args = input_map.into_iter().skip(1).collect::<Vec<String>>();

    (command, args)
}

fn echo(args: Vec<String>) {
    if args.is_empty() {
        println!();
    } else {
        println!("{}", args.join(" "));
    }
}

fn type_fn(input: &str) {
    match input {
        "echo" => println!("echo is a shell builtin"),
        "exit" => println!("exit is a shell builtin"),
        _ => println!("{}: not found", input),
    }
}
