use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]
pub enum Errors {
    InvalidKeyFormat(String),
}

impl Error for Errors {}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::InvalidKeyFormat(msg) => write!(f, "{}", msg)
        }
    }
}
