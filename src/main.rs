mod config;
mod db;
mod handlers;
mod models;
mod routes;

use actix_web::{middleware::Logger, App, HttpServer};
use config::Config;
use figment::{providers::Env, Figment};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env
    dotenvy::dotenv()?;

    // Build config by merging environment variables with Config::default()
    let config: Config = Figment::from(Config::default())
        .merge(Env::prefixed("ARCHIVIST_"))
        .extract()?;

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(config.log_level));

    HttpServer::new(|| App::new().configure(routes::routes).wrap(Logger::default()))
        .bind((config.addr, config.port))?
        .run()
        .await
        .map_err(|e| e.into())
}
