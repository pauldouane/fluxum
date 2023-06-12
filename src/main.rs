use crate::config::Config;
use crate::logger::Logger;

mod config;
mod error;
mod job;
mod logger;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut logger = Logger::new();
    logger.log(&"test", "error");
    let mut c = Config { jobs: vec![] };
    match c.get_jobs_by_config().await {
        Ok(_) => print!("All jobs has been charged"),
        Err(err) => logger.log(&"test", "error"),
    }
    Ok(())
}
