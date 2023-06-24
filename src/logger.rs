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
            "trace" => {
                self.stdout
                    .write(utils::format_trace(message).as_bytes())
                    .expect("Error in format trace log");
            }
            &_ => {
                self.stdout
                    .write(utils::format_error("test").as_bytes())
                    .expect("Error in format info log");
            }
        }
    }
}
