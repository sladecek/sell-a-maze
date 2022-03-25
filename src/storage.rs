use std::io;
use cloud_storage::object::Object;
use uuid::Uuid;
use crate::job::Job;

const BUCKET_NAME: &str = "sell-a-maze1923";

#[derive(Debug)]


pub struct Storage {}

impl Storage {

    pub async fn save_job_async(id: Uuid, job: &Job) -> io::Result<()>
     {
        let content = serde_json::to_string(job)?.as_bytes().to_vec();
        Object::create(BUCKET_NAME, content, &id.to_string(), "application/json").await.unwrap();
        Ok(())
    }

    pub fn save_job(id: Uuid, job: &Job) -> io::Result<()>
     {
        let content = serde_json::to_string(job)?.as_bytes().to_vec();
        Object::create_sync(BUCKET_NAME, content, &id.to_string(), "application/json").unwrap();
        Ok(())
    }

    pub async fn load_job_async(id: Uuid) -> io::Result<Job> {
        let content = Object::download(BUCKET_NAME, &id.to_string()).await.unwrap();
        let json = String::from_utf8(content).unwrap();
        let result: Job = serde_json::from_str(&json)?;
        Ok(result)
    }

    pub fn load_job(id: Uuid) -> io::Result<Job> {
        let content = Object::download_sync(BUCKET_NAME, &id.to_string()).unwrap();
        let json = String::from_utf8(content).unwrap();
        let result: Job = serde_json::from_str(&json)?;
        Ok(result)
    }

    pub fn save_file(name: &str, content: Vec<u8>, mime_type: &str) {
        Object::create_sync(BUCKET_NAME, content, name, mime_type).unwrap();
        
    }

    pub async fn load_file_async(name: &str) -> io::Result<Vec<u8>> {
        let content = Object::download(BUCKET_NAME, name).await.unwrap();
        Ok(content)
    }

}
