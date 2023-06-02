use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
    HttpServer::new(|| App::new().service(hello).service(create))
        .bind(("127.0.0.1", 5000))?
        .run()
        .await
}
