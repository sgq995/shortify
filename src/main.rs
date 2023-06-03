use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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
    hash.into_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(healthcheck).service(create))
        .bind(("127.0.0.1", 5000))?
        .run()
        .await
}
