use std::env;
use std::path::PathBuf;

// pub fn get_current_working_dir() -> std::io::Result<PathBuf> {
//     env::current_dir()
// }

pub fn format_error(message: &str) -> String {
    format!("\x1b[91m[ERROR]\x1b[0m : {}\n", message)
}

pub fn format_info(message: &str) -> String {
    format!("\x1b[94m[INFO]\x1b[0m : {}\n", message)
}

pub fn format_warning(message: &str) -> String {
    format!("\x1b[93m[WARNING]\x1b[0m : {}\n", message)
}
