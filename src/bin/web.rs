use actix_files as fs;
use actix_web::{self, web::Data, middleware::Logger, App, HttpServer};
use env_logger;
use sell_a_maze::{handler, storage::GoogleJobStorage, queue::JobQueue};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let queue = Data::new(JobQueue::new());
    let storage = Data::new(GoogleJobStorage::new());

    std::env::set_var("RUST_LOG", "actix_web=debug");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::clone(&queue))
            .app_data(Data::clone(&storage))
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
