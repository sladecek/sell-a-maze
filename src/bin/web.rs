use actix_files as fs;
use actix_web::{self, middleware::Logger, web::Data, App, HttpServer};
use env_logger;
use sell_a_maze::{
    handler,
    job::{Job, State},
    maker::MazeMaker,
    queue::Queue,
    storage::Storage,
};
use std::thread::spawn;
use std::{collections::VecDeque, sync::Mutex};
use std::{thread, time};
use uuid::Uuid;

fn process_job(id: &Uuid, job: &mut Job) -> bool {
    if job.state == State::WaitingForPayment {
        log::info!("Payment confirmed (demo mode)");
        job.state = State::InProgress;
        return true;
    }
    let id_str = id.to_string();
    if job.state == State::InProgress {
        job.svg = format!("{}.svg", id_str);
        job.pdf = format!("{}.pdf", id_str);
        if job.guaranteed {
            job.maze_structure = format!("{}.mas", id_str);
            job.maze_instance = format!("{}.mai", id_str);
            job.protocol = format!("cairo_log_{}.txt", id_str);
        }
        let ok = MazeMaker::make(job);
        job.state = if ok { State::Done } else { State::Error };
        return true;
    }
    false
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("SERVICE_ACCOUNT", "work/secret-key.json");
    std::env::set_var("RUST_LOG", "actix_web=debug");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Worker thread makes mazes one by
    let queue = Queue {
        uids: Mutex::new(VecDeque::new()),
    };
    let queue_data = Data::new(queue);

    let sd2 = queue_data.clone();
    let _worker_thread = spawn(move || loop {

        let delay = time::Duration::from_millis(1);
        thread::sleep(delay);

        let item = sd2.uids.lock().unwrap().pop_front();
        if item.is_some() {
            let id = item.unwrap();
            let job_res = Storage::load_job(id);
            if job_res.is_err() {
                log::error!("Cannot load job {} from storage", id);
                continue;
            }
            let mut job = job_res.unwrap();
            let changed = process_job(&id, &mut job);
            if changed {
                if Storage::save_job(id, &job).is_err() {
                    log::error!("Cannot save job {} to storage", id);
                    continue;
                }
            }
            if job.is_in_progress() {
                sd2.uids.lock().unwrap().push_back(id);
            }
        }
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::clone(&queue_data))
            .service(handler::maze_post)
            .service(handler::maze_get)
            .service(handler::file_get)
            .service(handler::version_get)
            .service(
                fs::Files::new("/", "static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
    })
    .bind("0.0.0.0:2705")?
    .run()
    .await
}
