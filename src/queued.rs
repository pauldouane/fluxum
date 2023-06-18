use crate::job::Job;

pub struct Queued<'a> {
    jobs: Vec<&'a Job>,
}
