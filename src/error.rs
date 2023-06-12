use core::fmt;
use std::{error::Error, fmt::write};
#[derive(Debug)]

struct ParseConfigError {}
struct ConfigMissingProperty {}

#[derive(Debug)]
pub enum ConfigError {
    ParseConfigError(String),
    ConfigMissingProperty(usize),
    FileNotFoundError(tokio::io::Error),
}

pub enum ConfigErrorKind {
    ParseConfigError,
    ConfigMissingProperty,
    FileNotFoundError,
}

impl ConfigError {
    pub fn kind(&self) -> ConfigErrorKind {
        match self {
            ConfigError::ConfigMissingProperty(_) => ConfigErrorKind::ConfigMissingProperty,
            ConfigError::ParseConfigError(_) => ConfigErrorKind::ParseConfigError,
            ConfigError::FileNotFoundError(_) => ConfigErrorKind::FileNotFoundError,
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::ParseConfigError(message) => write!(f, "{}", message),
            ConfigError::ConfigMissingProperty(number_job) => {
                write!(
                    f,
                    "ConfigMissingProperty: a property is missing on the job number {}",
                    number_job
                )
            }
            ConfigError::FileNotFoundError(err) => {
                write!(f, "ConfigFileNotFoundError : {}\n", err)
            }
        }
    }
}

#[derive(Debug)]
pub struct LoggerError {
    details: String,
}

impl LoggerError {
    pub fn new(msg: &str) -> LoggerError {
        LoggerError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for LoggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for LoggerError {
    fn description(&self) -> &str {
        &self.details
    }
}
