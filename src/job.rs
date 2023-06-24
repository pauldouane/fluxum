use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::process::Output;
use std::str::from_utf8;
use std::sync::Arc;
use tokio::{
    process::Command,
    task::{self, JoinHandle},
};
use tokio::sync::Mutex;
use crate::logger::Logger;

#[derive(Debug)]
pub enum Status {
    Running,
    Queued,
    NoStatus,
}

#[derive(Debug)]
pub struct Job {
    pub name: Vec<u8>,
    pub id: Vec<u8>,
    pub run: Vec<u8>,
    pub status: Status,
}

impl Job {
    pub async fn execute(&mut self, logger: Arc<Mutex<Logger>>) -> Result<String, Box<dyn std::error::Error>> {
        let mut logger = logger.lock().await;
        let output = Command::new("sh")
            .arg("-c")
            .arg(String::from_utf8_lossy(&self.run).to_string())
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            logger.log(&format!("Job output: {}", stdout), "trace");
        } else {
            eprintln!("Command execution failed. Error: {}", stderr);
        }

        Ok(stdout)
    }
}