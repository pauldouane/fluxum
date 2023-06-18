use crate::job::Job;

pub struct Running<'a> {
    jobs: Vec<&'a Job>,
}
