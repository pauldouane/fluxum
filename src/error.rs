use core::fmt;

#[derive(Debug)]
pub enum ConfigError {
    FileNotFoundError(tokio::io::Error),
}

/*
pub enum ConfigErrorKind {
    ConfigMissingProperty,
    FileNotFoundError,
}
*/

/*
impl ConfigError {
    pub fn kind(&self) -> ConfigErrorKind {
        match self {
            ConfigError::FileNotFoundError(_) => ConfigErrorKind::FileNotFoundError,
        }
    }
}
*/

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFoundError(err) => {
                writeln!(f, "ConfigFileNotFoundError : {}", err)
            }
        }
    }
}

/*
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
*/
