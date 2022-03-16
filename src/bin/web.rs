use actix_web::{self, middleware::Logger, App, HttpServer};
use actix_files as fs;
use env_logger;
use sell_a_maze::handler;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
//    let db = Db::new();

//    let mut chlgs = ChallengeService::new(1, 2);
//    let ts = RealTimeSource {};
//    let (today_utc, today_jd) = ts.today();
//    chlgs.update_all(today_jd, &db).unwrap();
//    let a = std::sync::Arc::new(chlgs.map);

    std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                fs::Files::new("/static", "static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            //            .data(db.clone())
//            .data(a.clone())
//            .data(today_utc)
            // .service(handler::contract_current_get)
            // .service(handler::contract_get)
            // .service(handler::contracts_get)
            // .service(handler::proofs_post)
            // .service(handler::proof_feedback_post)
            // .service(handler::user_get)
            // .service(handler::user_events_get)
            // .service(handler::user_prover_cards_post)
            // .service(handler::users_post)
            .service(handler::version_get)
    })
    .bind("0.0.0.0:2705")?
    .run()
    .await
}
