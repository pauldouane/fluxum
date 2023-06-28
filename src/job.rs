use crate::Logger;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, ChildStderr, ChildStdout, Command};
use tokio::sync::{Mutex, MutexGuard};

#[derive(Debug)]
pub enum Status {
    Running,
    Succeed,
    Failed,
    NoStatus,
}

pub struct Job {
    pub name: Vec<u8>,
    pub id: Vec<u8>,
    pub run: Vec<u8>,
    pub status: Status,
    pub operator: Operator,
}

impl Job {
    pub async fn execute_operator(
        &mut self,
        logger: Arc<Mutex<Logger>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut logger: MutexGuard<Logger> = logger.lock().await;
        let run = String::from_utf8_lossy(&self.run).to_string();
        let mut child: Child = match self.operator.execute(run).await {
            Ok(child) => child,
            Err(err) => panic!("{}", err),
        };

        let stdout: ChildStdout = child.stdout.take().expect("Failed to capture stdout");
        let stderr: ChildStderr = child.stderr.take().expect("Failed to capture stderr");

        let stdout_reader: BufReader<ChildStdout> = BufReader::new(stdout);
        let stderr_reader: BufReader<ChildStderr> = BufReader::new(stderr);

        let mut stdout_lines_iter = stdout_reader.lines();
        while let Some(line_result) = stdout_lines_iter.next_line().await.transpose() {
            let line = match line_result {
                Ok(line) => line,
                Err(_err) => String::from(""),
            };
            logger.log(&line.to_string(), "stdout");
        }

        let mut stderr_lines_iter = stderr_reader.lines();
        while let Some(line_result) = stderr_lines_iter.next_line().await.transpose() {
            let line = match line_result {
                Ok(line) => line,
                Err(_err) => String::from(""),
            };
            logger.log(&line.to_string(), "sttderr");
        }

        let status = match child.wait().await {
            Ok(status) => status,
            Err(_) => panic!("Unable to get the status of the job"),
        };
        if status.success() {
            self.set_succeed(logger).await;
        } else {
            self.set_failed(logger).await;
        }

        Ok(())
    }

    pub async fn set_running(&mut self, logger: Arc<Mutex<Logger>>) {
        self.status = Status::Running;
        logger.lock().await.log(
            &format!(
                "Job {} status has been updated: Running",
                String::from_utf8_lossy(&self.name)
            ),
            "status",
        );
    }

    pub async fn set_succeed(&mut self, mut logger: MutexGuard<'_, Logger>) {
        self.status = Status::Succeed;
        logger.log(
            &format!(
                "Job {} status has been updated: Succeed",
                String::from_utf8_lossy(&self.name)
            ),
            "success",
        );
    }

    pub async fn set_failed(&mut self, mut logger: MutexGuard<'_, Logger>) {
        self.status = Status::Failed;
        logger.log(
            &format!(
                "Job {} status has been updated: Failed",
                String::from_utf8_lossy(&self.name)
            ),
            "error",
        );
    }
}

#[derive(Debug)]
pub enum Operator {
    Shell,
    Python,
    Docker,
    Go,
}

impl Operator {
    pub async fn execute(&mut self, run: String) -> Result<Child, Box<dyn std::error::Error>> {
        match self {
            Self::Shell => {
                let child = Command::new("sh")
                    .arg("-c")
                    .arg(run)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;
                Ok(child)
            }
            Self::Python => {
                let child = Command::new("python3")
                    .arg("-u")
                    .arg(run)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;
                Ok(child)
            }
            Operator::Docker => {
                let child = Command::new("docker")
                    .arg("build")
                    .arg("--progress=plain")
                    .arg("--log")
                    .arg(run)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;
                Ok(child)
            }
            Operator::Go => {
                let child = Command::new("go")
                    .arg("build")
                    .arg("-x")
                    .arg(run)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;
                Ok(child)
            }
        }
    }
}
