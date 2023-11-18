use std::error::Error;

/// Custom error enum representing different error scenarios.
#[derive(Debug)]
pub enum CustomError {
    /// Error indicating that a component is drawn outside the canvas boundaries.
    OutOfCanvas,
    /// Error indicating that no configuration is provided.
    NoConfigProvided,
    /// Error indicating that there is no component to draw.
    ThereIsNoComponent,
}

impl std::fmt::Display for CustomError {
    /// Implements the Display trait to format the error messages.
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
