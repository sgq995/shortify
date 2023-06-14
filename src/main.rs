use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub mod database;
pub mod models;
pub mod schema;

#[derive(Serialize)]
struct Helthcheck {
    code: u16,
    status: String,
}

#[get("/healthcheck")]
async fn healthcheck() -> Result<impl Responder> {
    Ok(web::Json(Helthcheck {
        code: 200,
        status: "OK".to_string(),
    }))
}

#[derive(Deserialize)]
struct UrlFormData {
    url: String,
}

#[post("/")]
async fn create(form: web::Form<UrlFormData>) -> impl Responder {
    let fingerprint = Sha256::digest(form.url.to_owned());
    let hash = bs58::encode(fingerprint);
    let hash = hash.into_string();
    hash[0..8].to_owned()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = database::connect();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(healthcheck)
            .service(create)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
