use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web};
use serde::Deserialize;

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
    form.url.to_owned()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(create))
        .bind(("127.0.0.1", 5000))?
        .run()
        .await
}
