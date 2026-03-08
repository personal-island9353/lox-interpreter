use crate::error::Error;
use crate::scanner::Scanner;

pub struct Runner {
    pub error: Error
}

impl Runner {
    pub fn new() -> Runner {
        Runner { error: Error::new() }
    }

    pub fn had_error(&self) -> bool {
        self.error.had_error()
    }

    pub fn run(&mut self, program: String) {
        let mut scanner = Scanner::new(program, &mut self.error);
        let tokens = scanner.scan_tokens();
        tokens.iter().for_each(|token| println!("{}", token));
    }

    pub fn reset_error(&mut self) {
        self.error.reset_error()
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[Line {line}] Error{where_}: {message}");
        self.error.register_error();
    }
}
