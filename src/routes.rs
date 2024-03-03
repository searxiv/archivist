use crate::handlers::{archive, meta, tasks};
use crate::models::{NewAuthor, NewPaperFull, NewSubject, Paper};
use actix_web::web;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        meta::ping,
        archive::post_paper,
        archive::get_status,
        archive::get_papers_from_day,
        tasks::get_task,
        tasks::get_status,
        tasks::post_day_as_task
    ),
    components(schemas(NewPaperFull, Paper, NewAuthor, NewSubject))
)]
struct ApiDoc;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(meta::ping)
        .service(
            utoipa_rapidoc::RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi())
                .path("/docs"),
        )
        .service(archive::post_paper)
        .service(archive::get_status)
        .service(archive::get_papers_from_day)
        .service(tasks::get_task)
        .service(tasks::get_status)
        .service(tasks::post_day_as_task);
}
