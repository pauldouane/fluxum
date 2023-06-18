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

    pub fn log(&mut self, message: &str, logtype: &str) {
        match logtype {
            "info" => {
                self.stdout
                    .write(utils::format_info(message).as_bytes())
                    .expect("Error in format info log");
            }
            "error" => {
                self.stdout
                    .write(utils::format_error(message).as_bytes())
                    .expect("Error in format info log");
            }
            "warning" => {
                self.stdout
                    .write(utils::format_warning(message).as_bytes())
                    .expect("Error in format warning log");
            }
            "warning" => {
                self.stdout
                    .write(utils::format_logs(message).as_bytes())
                    .expect("Error in format warning log");
            }
            &_ => {
                self.stdout
                    .write(utils::format_error("The type log is not specified").as_bytes())
                    .expect("Error in format info log");
            }
        }
    }
}
