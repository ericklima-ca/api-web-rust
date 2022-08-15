use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder, Result};
use env_logger::Env;
use serde::Serialize;

#[derive(Serialize)]
struct MyResponse {
    msg: String,
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    let obj = MyResponse {
        msg: "ok".to_string(),
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
