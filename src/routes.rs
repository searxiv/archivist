use actix_web::web;

use crate::handlers::{archive, meta, tasks};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ping").get(meta::ping))
        .service(
            web::scope("/archive")
                .service(web::resource("/").post(archive::post_paper))
                .service(web::resource("/status").get(archive::get_status))
                .service(web::resource("/{year}/{month}/{day}").get(archive::get_papers_from_day)),
        )
        .service(
            web::scope("/tasks")
                .service(web::resource("/").get(tasks::get_task))
                .service(web::resource("/status").get(tasks::get_status))
                .service(web::resource("/{year}/{month}/{day}").post(tasks::post_day_as_task)),
        );
}
