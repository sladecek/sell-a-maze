use std::io;

use uuid::Uuid;

use crate::job::Job;


trait JobStorage {
    fn save(id: Uuid, job: &Job) -> io::Result<()>;
    fn load(id: Uuid) -> io::Result<Job>;
}

#[derive(Debug)]

pub struct GoogleJobStorage {

}

impl GoogleJobStorage {
    pub fn new()->Self {
        GoogleJobStorage{}
    }
}
