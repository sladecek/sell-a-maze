use actix_web::{self, get, post, web, HttpResponse};
use std::str::FromStr;
use uuid::Uuid;

use crate::job::{Job, State};
use crate::queue::Queue;
use crate::storage::Storage;

#[post("/api/maze")]
pub async fn maze_post(queue: web::Data<Queue>, job: web::Json<Job>) -> HttpResponse {
    let id = Uuid::new_v4();
    let mut j: Job = (*job).clone();
    j.state = if j.guaranteed {
        State::WaitingForPayment
    } else {
        State::InProgress
    };

    log::info!("Maze request id={} {:?}", id, job);
    let sr = Storage::save_job_async(id, &job).await;
    if sr.is_ok() {
        queue.uids.lock().unwrap().push_back(id);
        HttpResponse::Ok().json(id.to_string())
    } else {
        log::error!("Cannot save job id={} {}", id, sr.unwrap_err());
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/api/maze/{id}")]
pub async fn maze_get(path: web::Path<String>) -> HttpResponse {
    let id_str = path.into_inner();
    let id = Uuid::from_str(&id_str);
    if id.is_err() {
        HttpResponse::BadRequest().finish()
    } else {
        let uuid = id.unwrap();
        let job_res = Storage::load_job_async(uuid).await;
        if job_res.is_err() {
            log::error!("Cannot load {} from cloud storage", uuid);
            HttpResponse::InternalServerError().finish()
        } else {
            HttpResponse::Ok().json(job_res.unwrap())
        }
    }
}

#[get("/api/file/{name}")]
pub async fn file_get(path: web::Path<String>) -> HttpResponse {
    let name_str = path.into_inner();
    let file_res = Storage::load_file_async(&name_str).await;
    if file_res.is_err() {
        log::error!("Cannot load {} from cloud storage", name_str);
        HttpResponse::InternalServerError().finish()
    } else {
        let mime_type = if name_str.ends_with(".svg") {
            "image/svg+xml"
        } else {
            "application/octet-stream"
        };
        HttpResponse::Ok()
            .content_type(mime_type)
            .header("accept-ranges", "bytes")
            .header("content-disposition", format!("attachment; filename=\"{}\"",name_str))
            .body(file_res.unwrap())
    }
}

#[get("/api/version")]
pub async fn version_get() -> std::io::Result<String> {
    Ok(format!("3.14"))
}
