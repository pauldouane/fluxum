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

pub fn format_trace(message: &str) -> String {
    format!("\x1b[93m[TRACE]\x1b[0m : {}", message)
}

pub fn format_status(message: &str) -> String {
    format!("\x1b[89m[STATUS]\x1b[0m : {}\n", message)
}

pub fn format_success(message: &str) -> String {
    format!("\x1b[92m[SUCCESS]\x1b[0m : {}\n", message)
}

pub fn format_stdout(message: &str) -> String {
    format!("\x1b[93m[STDOUT]\x1b[0m : {}\n", message)
}

pub fn format_stderr(message: &str) -> String {
    format!("\x1b[91m[STDERR]\x1b[0m : {}\n", message)
}
