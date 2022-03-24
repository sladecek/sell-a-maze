use std::io;
use cloud_storage::object::Object;
use uuid::Uuid;
use crate::job::Job;

const BUCKET_NAME: &str = "sell-a-maze1923";

#[derive(Debug)]


pub struct GoogleJobStorage {}

impl GoogleJobStorage {
    pub fn new() -> Self {
        GoogleJobStorage {}
    }
    
    pub async fn save_async(&self, id: Uuid, job: &Job) -> io::Result<()>
     {
        let content = serde_json::to_string(job)?.as_bytes().to_vec();
        Object::create(BUCKET_NAME, content, &id.to_string(), "application/json").await.unwrap();
        Ok(())
    }

    
    pub fn save(&self, id: Uuid, job: &Job) -> io::Result<()>
     {
        let content = serde_json::to_string(job)?.as_bytes().to_vec();
        Object::create_sync(BUCKET_NAME, content, &id.to_string(), "application/json").unwrap();
        Ok(())
    }

    pub fn load(&self, id: Uuid) -> io::Result<Job> {
     
        let content = Object::download_sync(BUCKET_NAME, &id.to_string()).unwrap();
        let json = String::from_utf8(content).unwrap();
        let result: Job = serde_json::from_str(&json)?;
        Ok(result)
    }
}
