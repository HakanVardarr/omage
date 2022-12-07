use std::error::Error;

#[derive(Debug)]
pub enum CustomError {
    BuildError,
    SaveError,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &CustomError::BuildError => {
                write!(f, "ERROR: There is something wrong in your build script!")
            }
            &CustomError::SaveError => {
                write!(f, "ERROR: There is something wrong in your save script!")
            }
        }
    }
}

impl Error for CustomError {}
