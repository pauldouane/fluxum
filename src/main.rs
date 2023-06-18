use std::future::Future;
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
                println!("{:?}", &job.run);
                let handle = tokio::spawn(async move {
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg(String::from_utf8_lossy(&job.run).to_string())
                        .output()
                        .await;
                    output
                });

                match handle.await.unwrap() {
                    Ok(output) => {
                        if output.status.success() {
                            let output_string = String::from_utf8_lossy(&output.stdout);
                            logger.log(&format!("{:?}", output_string), "success")
                        } else {
                            let error_string = String::from_utf8_lossy(&output.stderr);
                            logger.log(&format!("{:?}", error_string), "error")
                        }
                    }
                    Err(err) => {
                        logger.log(&format!("{:?}", err), "error")
                    }
                }
            }
            Ok(())
        }
        Err(err) => {
            logger.log(&format!("{}", err), "error");
            Err(err)
        }
    }.expect("TODO: panic message");
    Ok(())
}
