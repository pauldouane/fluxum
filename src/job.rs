use tokio::{
    process::Command,
    task::{self, JoinHandle},
};

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
