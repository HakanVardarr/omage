use std::error::Error;

#[derive(Debug)]
pub enum CustomError {
    OutOfCanvas,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &CustomError::OutOfCanvas => {
                write!(f, "ERROR: Out of canvas")
            }
        }
    }
}

impl Error for CustomError {}
