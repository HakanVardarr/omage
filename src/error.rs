use std::error::Error;

#[derive(Debug)]
pub enum CustomError {
    OutOfCanvas,
    NoConfigProvided,
    ThereIsNoComponent,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &CustomError::OutOfCanvas => {
                write!(f, "ERROR: Out of canvas")
            }
            &CustomError::NoConfigProvided => {
                write!(f, "ERROR: No config provided")
            }
            &CustomError::ThereIsNoComponent => {
                write!(f, "ERROR: There is no component")
            }
        }
    }
}

impl Error for CustomError {}
