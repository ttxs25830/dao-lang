use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SyntaxError {
    UnsolvableChar { offset: usize },
}
impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsolvableChar { offset } => {
                write!(f, "Cant resolve char at offset:{} to valid token", offset)
            }
        }
    }
}
impl Error for SyntaxError {}
