use actix_web::{get, post, web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::actions;
use crate::database;

#[derive(Serialize)]
struct Helthcheck {
    code: u16,
    status: String,
}

#[derive(Deserialize)]
struct CreateUrlFormData {
    url: String,
}

#[derive(Serialize)]
struct UrlResponse {
    url: String,
    hash: String,
}

#[get("/healthcheck")]
async fn healthcheck() -> Result<impl Responder> {
    Ok(web::Json(Helthcheck {
        code: 200,
        status: "OK".to_string(),
    }))
}

#[get("/{hash}")]
async fn read(
    pool: web::Data<database::DatabasePool>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let hash = path.into_inner();
    let created_url = web::block(move || {
        let mut conn = pool
            .get()
            .expect("Couldn't get database connection from pool");

        actions::select_url(&mut conn, hash)
    })
    .await?
    .map_err(actix_web::error::ErrorNotFound)?;

    Ok(web::Redirect::to(created_url.url).permanent())
}

#[post("/")]
async fn create(
    pool: web::Data<database::DatabasePool>,
    form: web::Form<CreateUrlFormData>,
) -> actix_web::Result<impl Responder> {
    let url = form.url.to_owned();

    let fingerprint = Sha256::digest(url.to_owned());
    let url_hash = bs58::encode(fingerprint);
    let url_hash = url_hash.into_string();
    let url_hash = url_hash[0..8].to_owned();

    let created_url = web::block(move || {
        let mut conn = pool
            .get()
            .expect("Couldn't get database connection from pool");

        actions::insert_url(&mut conn, url_hash, url)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(UrlResponse {
        url: created_url.url,
        hash: created_url.hash,
    }))
}
