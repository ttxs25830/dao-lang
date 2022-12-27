use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct SyntaxError {
    line: usize,
    reason: &'static str,
}
impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SyntaxError on line {}, {}", self.line, self.reason)
    }
}
impl Error for SyntaxError {}
impl SyntaxError {
    pub fn new(line: usize, reason: &'static str) -> Self {
        Self { line, reason }
    }
}
