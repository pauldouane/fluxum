use crate::error::LoggerError;
use crate::utils;
use std::io::{Stdout, Write};

fn print_ascii() {
    println!(
        " 
----------------------------------------------------------------------------------
| ███████████ █████       █████  █████ █████ █████ █████  █████ ██████   ██████  |
| ░░███░░░░░░█░░███       ░░███  ░░███ ░░███ ░░███ ░░███  ░░███ ░░██████ ██████  |
|  ░███   █ ░  ░███        ░███   ░███  ░░███ ███   ░███   ░███  ░███░█████░███  |
|  ░███████    ░███        ░███   ░███   ░░█████    ░███   ░███  ░███░░███ ░███  |
|  ░███░░░█    ░███        ░███   ░███    ███░███   ░███   ░███  ░███ ░░░  ░███  |
|  ░███  ░     ░███      █ ░███   ░███   ███ ░░███  ░███   ░███  ░███      ░███  |
|  █████       ███████████ ░░████████   █████ █████ ░░████████   █████     █████ |
| ░░░░░       ░░░░░░░░░░░   ░░░░░░░░   ░░░░░ ░░░░░   ░░░░░░░░   ░░░░░     ░░░░░  |
----------------------------------------------------------------------------------
		"
    );
}

pub struct Logger {
    stdout: Stdout,
}

impl Logger {
    pub fn new() -> Logger {
        print_ascii();
        return Logger {
            stdout: std::io::stdout(),
        };
    }

    pub fn log(&mut self, message: &str, logtype: &str) -> Result<(), LoggerError> {
        match logtype {
            "info" => {
                self.stdout
                    .write(utils::format_info(message).as_bytes())
                    .expect("Error in format info log");
                Ok(())
            }
            "error" => {
                self.stdout.write(utils::format_error(&"").as_bytes());
                panic!("{}", utils::format_error(message));
            }
            "warning" => {
                self.stdout
                    .write(utils::format_warning(message).as_bytes())
                    .expect("Error in format warning log");
                Ok(())
            }
            &_ => Err(LoggerError::new(
                "The type specified at the log level does not exist",
            )),
        }
    }
}
