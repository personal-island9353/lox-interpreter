mod runner;
mod scanner;
mod error;

use crate::runner::Runner;
use std::env;
use std::io::Write;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 2 {
        println!("Usage: lox [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) {
    let program =
        std::fs::read_to_string(path).expect(format!("Failed to read file {path}").as_str());
    let mut runner = Runner::new();
    runner.run(program);
    if runner.had_error() {
        std::process::exit(65);
    }
}

fn run_prompt() {
    loop {
        display_prompt();
        let line = read_line();
        let mut runner = Runner::new();
        runner.run(line);
        runner.reset_error();
    }
}

fn display_prompt() {
    print!("> ");
    std::io::stdout().flush().expect("Failed to flush stdout");
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    line
}
