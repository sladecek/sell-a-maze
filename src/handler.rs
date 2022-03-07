use actix_web::{
    self, error, get, http::header::Header, post, web, Error, HttpRequest, HttpResponse,
};
//use actix_web_httpauth::headers::authorization::{Authorization, Basic};
use log::info;

/*
#[get("/contracts")]
pub async fn contracts_get(db: web::Data<Db>) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().json(db.list_terms_of_service()))
}

#[get("/contract/current")]
pub async fn contract_current_get(db: web::Data<Db>) -> Result<HttpResponse, actix_web::Error> {
    info!("ac");
    db.get_current_terms_of_service()
        .map(|ok| HttpResponse::Ok().json(ok))
        .map_err(|_| error::ErrorInternalServerError(""))
}

#[post("/proofs")]
pub async fn proofs_post(
    db: web::Data<Db>,
    chm: web::Data<ChallengeMap>,
    req: HttpRequest,
    now: web::Data<DateTime>,
    p: web::Json<api::Proof>,
) -> Result<HttpResponse, Error> {
    let calling_user = find_caller(&db, &req, &now)?;
    verify_proof(&db, &chm, &RealTimeSource {}, &p, &calling_user)
        .map(|ok| HttpResponse::Ok().json(ok))
        .map_err(|_| error::ErrorInternalServerError(""))
}

#[post("/proof/{proofId}/feedback")]
pub async fn proof_feedback_post(
    db: web::Data<Db>,
    #[allow(non_snake_case)] web::Path(proofId): web::Path<String>,
    req: HttpRequest,
    now: web::Data<DateTime>,
    fb: web::Json<api::Feedback>,
) -> Result<HttpResponse, Error> {
    let calling_user = find_caller(&db, &req, &now)?;
    post_proof_feedback(&db, &RealTimeSource {}, &calling_user, &proofId, &fb)
        .map(|ok| HttpResponse::Ok().json(ok))
        .map_err(|_| error::ErrorInternalServerError(""))
}

#[get("/contract/{digest}")]
pub async fn contract_get(
    db: web::Data<Db>,
    web::Path(digest): web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    db.get_terms_of_service(&digest)
        .map(|o| {
            o.map_or(HttpResponse::NotFound().finish(), |ok| {
                HttpResponse::Ok().json(ok)
            })
        })
        .map_err(|_| error::ErrorInternalServerError(""))
}

#[get("/user/{userId}")]
pub async fn user_get(
    db: web::Data<Db>,
    req: HttpRequest,
    now: web::Data<DateTime>,
    #[allow(non_snake_case)] web::Path(userId): web::Path<String>,
) -> Result<HttpResponse, Error> {
    info!("user_get/{}", userId);
    let calling_user = find_caller(&db, &req, &now)?;
    // TODO smi to spustit administrator nebo userId sam na sebe
    if userId == calling_user.u.user_id {
        Ok(HttpResponse::Ok().json(api::User::from(calling_user)))
    } else {
        Err(error::ErrorUnauthorized(""))
    }
}

#[post("/users")]
pub async fn users_post(
    db: web::Data<Db>,
    rq: web::Json<api::RegistrationRequest>,
    now: web::Data<DateTime>,
) -> Result<HttpResponse, Error> {
    new_user(&db, &rq, &now, None)
        .map(|ok| HttpResponse::Ok().json(ok))
        .map_err(|_| error::ErrorInternalServerError(""))
}

#[get("/user/{userId}/events")]
pub async fn user_events_get(
    db: web::Data<Db>,
    req: HttpRequest,
    now: web::Data<DateTime>,
    #[allow(non_snake_case)] web::Path(userId): web::Path<String>,
    paging: web::Query<Paging>,
) -> Result<HttpResponse, Error> {
    info!("user_events_get");
    let _calling_user = find_caller(&db, &req, &now)?;
    // TODO smi to spustit administrator nebo userId sam na sebe
    list_events(&db, &userId, &paging)
        .map(|ok| HttpResponse::Ok().json(ok))
        .map_err(|_| error::ErrorInternalServerError(""))
}

#[post("/user/{userId}/prover/cards")]
pub async fn user_prover_cards_post(
    db: web::Data<Db>,
    req: HttpRequest,
    now: web::Data<DateTime>,
    card: web::Json<api::Card>,
    #[allow(non_snake_case)] web::Path(userId): web::Path<String>,
) -> Result<HttpResponse, Error> {
    info!("user_prover_cards_post");
    let calling_user = find_caller(&db, &req, &now)?;

    new_card(&db, &card, calling_user, &userId)
        .map(|ok| HttpResponse::Ok().json(ok))
        .map_err(|_| error::ErrorInternalServerError(""))
}
*/
#[get("/version")]
pub async fn version_get() -> std::io::Result<String> {
    Ok(format!("3.14"))
}
/*
fn find_caller(
    db: &Db,
    rq: &HttpRequest,
    now: &DateTime,
) -> crate::error::Result<domain::user::UserWithState> {
    let auth = Authorization::<Basic>::parse(rq)?;
    let name = auth.as_ref().user_id();
    let password = auth.as_ref().password();
    if password.is_none() {
        return Err(crate::error::Error::new_not_authorised());
    }
    let user = db.find_authorized_user(name, password.unwrap())?;
    Ok(domain::user::UserWithState::new_from_user(user, now))
}
*/