use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;

pub mod actions;
pub mod database;
pub mod models;
pub mod routes;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let pool = database::connect();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(routes::healthcheck)
            .service(routes::read)
            .service(routes::create)
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
