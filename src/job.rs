use std::fmt::format;
use crate::Logger;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::Mutex;

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
    ) -> Result<String, Box<dyn std::error::Error>> {
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
            logger.log(&stderr, "error");
        }

        Ok(stdout)
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
        logger: Arc<Mutex<Logger>>,
    ) {
        self.status = Status::Succeed;
        logger.lock().await.log(
            &format!("Job {} status has been updated: Succeed", String::from_utf8_lossy(&self.name)),
            "status"
        );
    }

    pub async fn set_failed(
        &mut self,
        logger: Arc<Mutex<Logger>>,
    ) {
        self.status = Status::Failed;
        logger.lock().await.log(
            &format!("Job {} status has been updated: Failed", String::from_utf8_lossy(&self.name)),
            "status"
        );
    }
}
