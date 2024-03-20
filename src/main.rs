mod config;
mod db;
mod handlers;
mod models;
mod routes;

use actix_web::{
    middleware::{Compress, Logger},
    web::{Data, JsonConfig},
    App, HttpServer,
};
use clokwerk::TimeUnits;
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

    // Connect to database
    let db_url = std::env::var("DATABASE_URL")?;
    let db = std::sync::Arc::new(db::DBConnection::new(&db_url).await?);
    let db_cloned = db.clone();

    // Create and start scheduler
    let mut scheduler = clokwerk::AsyncScheduler::new();
    scheduler
        .every(config.task_duration_check_seconds.seconds())
        .run(move || {
            let threshold = config.task_duration_threshold_seconds;
            let db = db_cloned.clone();
            async move {
                let _ = db.revert_long_running_tasks(threshold).await;
            }
        });
    tokio::spawn(async move {
        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    // Start web server
    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .app_data(Data::new((*db).clone()))
            .app_data(JsonConfig::default().limit(1024 * 1024 * 1024))
            .configure(routes::routes)
            .wrap(Logger::default())
    })
    .bind((config.addr, config.port))?
    .run()
    .await?;

    Ok(())
}
