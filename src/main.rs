use std::process::Stdio;
use std::str::from_utf8;
use std::thread::JoinHandle;

use job::{Job, Status};
use tokio::process::Command;
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
    let mut logger = Logger::new();
    let mut config = Config { jobs: vec![] };
    match config.get_jobs_by_config(&mut logger).await {
        Ok(_) => {
            logger.log(&format!("Number of jobs : {}", config.jobs.len()), "info");
            for mut job in config.jobs {
                job.status = Status::Running;
                let handle = tokio::spawn(async move {
                    let output = Command::new("ls").output().await.unwrap();
                    output.stdout
                });

                let out = handle.await.unwrap();
                println!("{:?}", std::str::from_utf8(&out))
            }
            Ok(())
        }
        Err(err) => {
            logger.log(&format!("{}", err), "error");
            Err(err)
        }
    };
    Ok(())
}
