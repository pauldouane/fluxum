use std::fmt::format;
use std::process::Stdio;
use std::str::from_utf8;
use std::sync::Arc;
use std::thread::JoinHandle;

use job::{Job, Status};
use tokio::process::Command;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tokio::task;

use crate::config::Config;
use crate::logger::Logger;

mod config;
mod error;
mod job;
mod logger;
mod queued;
mod running;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut logger_mutex = Arc::new(Mutex::new(Logger::new())); // Create an Arc<Mutex<Logger>> for sharing the logger
    let mut config = Config { jobs: vec![] };
    let mut handles = vec![];
    config.get_jobs_by_config(logger_mutex.clone()).await.expect("Unable to get jobs");
    for mut job in config.jobs {
        let logger = Arc::clone(&logger_mutex); // Clone the Arc to increase reference count
        let handle = tokio::spawn(async move {
            job.execute(logger).await.expect("TODO: panic message");
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}
