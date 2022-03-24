use actix_web::{self, get, post, web, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::job::{Job, State};
use crate::queue::Queue;
use crate::storage::GoogleJobStorage;

#[post("/api/maze")]
pub async fn maze_post(
    queue: web::Data<Queue>,
    storage: web::Data<GoogleJobStorage>,
    job: web::Json<Job>,
) -> HttpResponse {
    let id = Uuid::new_v4();
    let mut j: Job = (*job).clone();
    j.state = if j.guaranteed { State::WaitingForPayment } else { State::InProgress }; 
    
    log::info!("Maze request id={} {:?}", id, job);
    let sr = storage.save_async(id, &job).await;
    if sr.is_ok() {
        queue.uids.lock().unwrap().push_back(id);
        HttpResponse::Ok()
    } else {
        log::error!("Cannot save job id={} {}", id, sr.unwrap_err());
        HttpResponse::InternalServerError()
    }
    .finish()
}

#[get("/api/maze/{id}")]
pub async fn maze_get(_storage: web::Data<GoogleJobStorage>, _req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/api/version")]
pub async fn version_get() -> std::io::Result<String> {
    Ok(format!("3.14"))
}
