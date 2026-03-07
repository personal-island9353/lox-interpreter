pub struct Error {
    had_error: bool
}

impl Error {
    pub fn new() -> Error {
        Error { had_error: false }
    }

    pub fn had_error(&self) -> bool {
        return self.had_error;
    }

    pub fn reset_error(&mut self) {
        self.had_error = false;
    }

    pub fn register_error(&mut self) {
        self.had_error = true
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[Line {line}] Error{where_}: {message}");
        self.had_error = true;
    }
}