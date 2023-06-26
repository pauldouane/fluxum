use crate::Logger;
use async_trait::async_trait;
use std::fmt::format;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader, Lines};
use tokio::process::{ChildStderr, ChildStdout, Command};
use tokio::sync::{Mutex, MutexGuard};

#[async_trait]
pub trait Job {
    async fn execute();
}

#[derive(Debug)]
pub enum Status {
    Running,
    Succeed,
    Failed,
    NoStatus,
}

#[derive(Debug)]
pub struct ShellJob {
    pub name: Vec<u8>,
    pub id: Vec<u8>,
    pub run: Vec<u8>,
    pub status: Status,
}

impl ShellJob {
    pub async fn execute(
        &mut self,
        logger: Arc<Mutex<Logger>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("coucou");
        // let mut logger: MutexGuard<Logger> = logger.lock().await;
        // let mut child = Command::new("sh")
        //     .arg("-c")
        //     .arg(String::from_utf8_lossy(&self.run).to_string())
        //     .stdout(Stdio::piped())
        //     .stderr(Stdio::piped())
        //     .spawn()?;

        // let stdout: ChildStdout = child.stdout.take().expect("Failed to capture stdout");
        // let stderr: ChildStderr = child.stderr.take().expect("Failed to capture stderr");

        // let stdout_reader: BufReader<ChildStdout> = BufReader::new(stdout);
        // let stderr_reader: BufReader<ChildStderr> = BufReader::new(stderr);

        // let mut stdout_lines_iter = stdout_reader.lines();
        // while let Some(line_result) = stdout_lines_iter.next_line().await.transpose() {
        //     let line = line_result?;
        //     logger.log(&format!("{}", line), "stdout");
        // }

        // let mut stderr_lines_iter = stderr_reader.lines();
        // while let Some(line_result) = stderr_lines_iter.next_line().await.transpose() {
        //     let line = line_result?;
        //     logger.log(&format!("{}", line), "sttderr");
        // }

        // let status = child.wait().await?;
        // if status.success() {
        //     self.set_succeed(logger).await;
        // } else {
        //     self.set_failed(logger).await;
        // }

        Ok(())
    }
}
