use crate::models::{NewPaper, Paper};
use actix_web::{
    get, post,
    web::{Json, Path},
    Responder,
};

#[utoipa::path(
    responses(
        (status = 200, description = "Get stats about archive")
    )
)]
#[get("/archive/status")]
pub async fn get_status() -> impl Responder {
    "get_status is not implemented"
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get all paper submitted in this day", body = [Paper])
    ),
    params(
        ("year",),
        ("month",),
        ("day",)
    ),
)]
#[get("/archive/{year}/{month}/{day}")]
pub async fn get_papers_from_day(date: Path<(i32, i32, i32)>) -> impl Responder {
    "get_papers_from_day is not implemented"
}

#[utoipa::path(
    request_body = NewPaper,
    responses(
        (status = 201, description = "Add new paper into archive"),
        (status = 400, description = "Body cannot be deserialized")
    )
)]
#[post("/archive")]
pub async fn post_paper(paper: Json<NewPaper>) -> impl Responder {
    "post_paper is not implemented"
}
