use actix_files as fs;
use actix_web::{self, middleware::Logger, web::Data, App, HttpServer};
use env_logger;
use sell_a_maze::{handler, job::{Job, State}, storage::GoogleJobStorage, queue::Queue, maker::MazeMaker};
use std::{collections::VecDeque, sync::Mutex};
use std::thread::spawn;
use std::{thread, time};

fn process_job(job: &mut Job) -> bool {
    if job.state == State::WaitingForPayment {
        log::info!("Payment confirmed (demo mode)");
        job.state = State::InProgress;
        return true
    }
    if job.state == State::InProgress {
        MazeMaker::make(job,"work/");
        job.state = State::Done;
    }
    false

}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("SERVICE_ACCOUNT", "work/grand-kingdom-344016-e5104993a6aa.json");
    std::env::set_var("RUST_LOG", "actix_web=debug");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let web_storage = Data::new(GoogleJobStorage::new());

    // Worker thread makes mazes one by 
    let queue = Queue {uids: Mutex::new(VecDeque::new())};
    let queue_data = Data::new(queue);

    let sd2 = queue_data.clone();
    let _worker_thread = spawn(move || {
        let worker_storage = GoogleJobStorage::new();

        loop {
            
            let item = sd2.uids.lock().unwrap().pop_front();
            if item.is_some() {
                let id = item.unwrap();
                let job_res = worker_storage.load(id);
                if job_res.is_err() {
                    log::error!("Cannot load job {} from storage", id);
                    continue;
                }
                let mut job = job_res.unwrap();
                let changed = process_job(&mut job);
                if changed {
                    if worker_storage.save(id, &job).is_err() {
                        log::error!("Cannot save job {} to storage", id);
                        continue;             
                    }
                }
                if job.is_in_progress() {
                    sd2.uids.lock().unwrap().push_back(id);

                    let ten_millis = time::Duration::from_millis(10);
                    thread::sleep(ten_millis)
                }
            }
        }
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::clone(&queue_data))
            .app_data(Data::clone(&web_storage))
            .service(handler::maze_post)
            .service(handler::maze_get)
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
