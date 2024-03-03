use actix_web::{
    get, post,
    web::{Json, Path},
    Responder,
};

#[utoipa::path(
    responses(
        (status = 200, description = "Get next task"),
        (status = 404, description = "No tasks available")
    )
)]
#[get("/tasks")]
pub async fn get_task() -> impl Responder {
    "get_task is not implemented"
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get info about task queue")
    )
)]
#[get("/status")]
pub async fn get_status() -> impl Responder {
    "get_status is not implemented"
}

#[utoipa::path(
    responses(
        (status = 201, description = "Create new task to scrape specific day")
    ),
    params(
        ("year",),
        ("month",),
        ("day",)
    ),
)]
#[post("/{year}/{month}/{day}")]
pub async fn post_day_as_task(date: Path<(i32, i32, i32)>) -> impl Responder {
    "post_day_as_task is not implemented"
}
