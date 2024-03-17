use std::fmt;

#[derive(Debug)]
pub enum HiramuError {
    Http(reqwest::Error),
    Io(std::io::Error),
    // Add more error types as needed
}

impl fmt::Display for HiramuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HiramuError::Http(e) => write!(f, "HTTP error: {}", e),
            HiramuError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for HiramuError {}

impl From<reqwest::Error> for HiramuError {
    fn from(err: reqwest::Error) -> HiramuError {
        HiramuError::Http(err)
    }
}

impl From<std::io::Error> for HiramuError {
    fn from(err: std::io::Error) -> HiramuError {
        HiramuError::Io(err)
    }
}