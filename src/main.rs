use std::sync::Arc;

use job::Status;
use tokio::sync::Mutex;

use crate::config::Config;
use crate::logger::Logger;

mod config;
mod error;
mod job;
mod logger;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let logger_mutex: Arc<Mutex<Logger>> = Arc::new(Mutex::new(Logger::new()));
    let mut config = Config { jobs: vec![] };
    let logger: Arc<Mutex<Logger>> = Arc::clone(&logger_mutex);
    config
        .get_jobs_by_config(logger)
        .await
        .expect("Unable to get jobs");
    for mut job in config.jobs {
        let logger: Arc<Mutex<Logger>> = Arc::clone(&logger_mutex);
        job.set_running(logger).await;
        let logger_spawn: Arc<Mutex<Logger>> = Arc::clone(&logger_mutex);
        let handle = tokio::spawn(async move {
            job.execute_operator(logger_spawn).await;
        });
        let _ = handle.await;
    }

    Ok(())
}
