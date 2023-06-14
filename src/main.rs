use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use diesel::{expression_methods::ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
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

fn insert_url(
    conn: &mut SqliteConnection,
    hash: String,
    url: String,
) -> diesel::QueryResult<models::Url> {
    use crate::schema::urls::dsl;

    let created_url = dsl::urls
        .filter(dsl::hash.eq(&hash))
        .first::<models::Url>(conn);
    let created_url = match created_url {
        Ok(result) => result,
        _ => {
            let new_url = models::CreateUrl {
                hash: &hash,
                url: &url,
            };

            diesel::insert_into(dsl::urls)
                .values(new_url)
                .execute(conn)
                .expect("Error inserting url");

            models::Url {
                id: 0,
                hash: hash,
                url: url,
            }
        }
    };

    Ok(created_url)
}

#[derive(Deserialize)]
struct UrlFormData {
    url: String,
}

#[post("/")]
async fn create(
    pool: web::Data<database::DatabasePool>,
    form: web::Form<UrlFormData>,
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

        insert_url(&mut conn, url_hash, url)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(created_url.hash))
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
