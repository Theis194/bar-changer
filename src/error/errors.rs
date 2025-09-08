use std::fmt;
use std::io;

#[derive(Debug)]
pub enum BarChangerError {
    Io(io::Error),
    ConfigNotFound(String),
    InvalidInput(String),
    StyleNotFound(String),
    PermissionDenied(String),
    Serialization(String),
    DeSerialization(String),
    Custom(String),
}

impl std::error::Error for BarChangerError {}

impl fmt::Display for BarChangerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BarChangerError::Io(e) => write!(f, "IO error: {}", e),
            BarChangerError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            BarChangerError::StyleNotFound(msg) => write!(f, "Style not found: {}", msg),
            BarChangerError::ConfigNotFound(msg) => write!(f, "Config not found: {}", msg),
            BarChangerError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            BarChangerError::Serialization(msg) => write!(f, "Failed serializing: {}", msg),
            BarChangerError::DeSerialization(msg) => write!(f, "Failed deserializing: {}", msg),
            BarChangerError::Custom(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl From<io::Error> for BarChangerError {
    fn from(error: io::Error) -> Self {
        BarChangerError::Io(error)
    }
}

impl From<String> for BarChangerError {
    fn from(error: String) -> Self {
        BarChangerError::Custom(error)
    }
}

impl From<&str> for BarChangerError {
    fn from(error: &str) -> Self {
        BarChangerError::Custom(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, BarChangerError>;

pub trait Context<T, E> {
    fn context(self, context: impl Into<String>) -> Result<T>;
}

impl<T, E: Into<BarChangerError>> Context<T, E> for std::result::Result<T, E> {
    fn context(self, context: impl Into<String>) -> Result<T> {
        self.map_err(|e| {
            let mut error: BarChangerError = e.into();
            if let BarChangerError::Custom(msg) = &mut error {
                *msg = format!("{}: {}", context.into(), msg);
            }
            error
        })
    }
}
