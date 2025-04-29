use sqrl_client::error::SqrlError;
use std::fmt;

pub(crate) struct CliError {
    error_message: String,
}

impl CliError {
    /// Create a new CliError with the string as error message
    pub fn new(error: String) -> Self {
        CliError {
            error_message: error,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl fmt::Debug for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl std::error::Error for CliError {}

impl From<SqrlError> for CliError {
    fn from(error: SqrlError) -> Self {
        CliError::new(error.to_string())
    }
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError::new(error.to_string())
    }
}
