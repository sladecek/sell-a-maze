
use actix_web::{
    self, post, web, HttpRequest, HttpResponse, get,
};
use crate::job::Job;
//use log::info;
use crate::queue::JobQueue;
use crate::storage::{GoogleJobStorage};

#[post("/api/maze")]
pub async fn maze_post(
    _queue: web::Data<JobQueue>,
    _storage: web::Data<GoogleJobStorage>,
    _info: web::Json<Job>
) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/api/maze/{id}")]
pub async fn maze_get(
    _queue: web::Data<JobQueue>,
    _storage: web::Data<GoogleJobStorage>,
    _req: HttpRequest,
) -> HttpResponse {

    HttpResponse::Ok().finish()
}

#[get("/api/version")]
pub async fn version_get() -> std::io::Result<String> {
    Ok(format!("3.14"))
}
