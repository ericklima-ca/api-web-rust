pub mod controllers;
pub mod models;

use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use controllers::score_controller;
use env_logger::Env;

#[get("/ping")]
async fn index() -> impl Responder {
    "pong"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(web::scope("/scores").service(score_controller::get_score))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
