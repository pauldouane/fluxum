use std::fmt::format;
use std::process::Stdio;
use crate::Logger;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader, Lines};
use tokio::process::{ChildStderr, ChildStdout, Command};
use tokio::sync::{Mutex, MutexGuard};

#[derive(Debug)]
pub enum Status {
    Running,
    Succeed,
    Failed,
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
    pub async fn execute(
        &mut self,
        logger: Arc<Mutex<Logger>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut logger:MutexGuard<Logger> = logger.lock().await;
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(String::from_utf8_lossy(&self.run).to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout:ChildStdout = child.stdout.take().expect("Failed to capture stdout");
        let stderr:ChildStderr = child.stderr.take().expect("Failed to capture stderr");

        let stdout_reader: BufReader<ChildStdout> = BufReader::new(stdout);
        let stderr_reader: BufReader<ChildStderr> = BufReader::new(stderr);

        let mut stdout_lines_iter = stdout_reader.lines();
        while let Some(line_result) = stdout_lines_iter.next_line().await.transpose() {
            let line = line_result?;
            println!("[STDOUT] {}", line);
        }

        let mut stderr_lines_iter = stderr_reader.lines();
        while let Some(line_result) = stderr_lines_iter.next_line().await.transpose() {
            let line = line_result?;
            println!("[STDERR] {}", line);
        }

        let status = child.wait().await?;
        if status.success() {
            self.set_succeed(logger).await;
        } else {
            println!("Command failed with exit code: {}", status.code().unwrap_or(-1));
        }

        Ok(())
    }

    pub async fn set_running(
        &mut self,
        logger: Arc<Mutex<Logger>>,
    ) {
        self.status = Status::Running;
        logger.lock().await.log(
            &format!("Job {} status has been updated: Running", String::from_utf8_lossy(&self.name)),
            "status"
        );
    }

    pub async fn set_succeed(
        &mut self,
        mut logger: MutexGuard<'_, Logger>,
    ) {
        self.status = Status::Succeed;
        logger.log(
            &format!("Job {} status has been updated: Succeed", String::from_utf8_lossy(&self.name)),
            "success"
        );
    }

    pub async fn set_failed(
        &mut self,
        mut logger: MutexGuard<'_, Logger>,
    ) {
        self.status = Status::Failed;
        logger.log(
            &format!("Job {} status has been updated: Failed", String::from_utf8_lossy(&self.name)),
            "error"
        );
    }
}
