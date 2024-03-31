use crate::handlers::{archive, tasks};
use crate::models;
use actix_web::web;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        archive::get_status,
        archive::get_papers_from_day,
        tasks::get_task,
        tasks::get_stats,
        tasks::post_day_as_task,
        tasks::post_month_as_task,
        tasks::post_year_as_task,
        tasks::submit_task,
    ),
    components(schemas(
        models::NewPaperFull,
        models::Paper,
        models::NewAuthor,
        models::NewSubject,
        models::ArchiveStats,
        models::NewTask,
        models::TasksStats,
        models::TaskSubmission,
    )),
    tags(
        (name = "tasks", description = "Tasks management api."),
        (name = "archive", description = "Archive management api."),
    )
)]
struct ApiDoc;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        utoipa_rapidoc::RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi())
            .path("/docs"),
    )
    .service(archive::get_status)
    .service(archive::get_papers_from_day)
    .service(tasks::get_task)
    .service(tasks::get_stats)
    .service(tasks::post_day_as_task)
    .service(tasks::post_month_as_task)
    .service(tasks::post_year_as_task)
    .service(tasks::submit_task);
}
