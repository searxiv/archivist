mod db;
mod handlers;
mod models;
mod routes;

use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| App::new().configure(routes::routes).wrap(Logger::default()))
        .bind(("0.0.0.0", 9000))?
        .run()
        .await
}
