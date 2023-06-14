use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use diesel::{prelude::*, sqlite::SqliteConnection};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;

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
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let sqlite_connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    HttpServer::new(|| App::new().service(healthcheck).service(create))
        .bind(("127.0.0.1", 5000))?
        .run()
        .await
}
