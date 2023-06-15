use actix_web::{web, App, HttpServer};

pub mod actions;
pub mod database;
pub mod models;
pub mod routes;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = database::connect();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::healthcheck)
            .service(routes::create)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
