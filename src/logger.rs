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
        Logger {
            stdout: std::io::stdout(),
        }
    }

    pub fn log(&mut self, message: &str, logtype: &str) -> usize {
        match logtype {
            "info" => {
                self.stdout
                    .write(utils::format_info(message).as_bytes()).unwrap()
            }
            "error" => {
                self.stdout
                    .write(utils::format_error(message).as_bytes()).unwrap()
            }
            "warning" => {
                self.stdout
                    .write(utils::format_warning(message).as_bytes()).unwrap()
            }
            "trace" => {
                self.stdout
                    .write(utils::format_trace(message).as_bytes()).unwrap()
            }
            "status" => {
                self.stdout
                    .write(utils::format_status(message).as_bytes()).unwrap()
            }
            &_ => {
                self.stdout
                    .write(utils::format_error("test").as_bytes()).unwrap()
            }
        }
    }
}
