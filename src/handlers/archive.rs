use crate::{
    db,
    models::{NewPaper, NewPaperFull},
};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
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
pub async fn get_papers_from_day(_date: Path<(i32, i32, i32)>) -> impl Responder {
    "get_papers_from_day is not implemented"
}

#[utoipa::path(
    request_body = NewPaperFull,
    responses(
        (status = 201, description = "Add new paper into archive"),
        (status = 400, description = "Body cannot be deserialized")
    )
)]
#[post("/archive")]
pub async fn post_paper(db: Data<db::DBConnection>, paper: Json<NewPaperFull>) -> impl Responder {
    let new_paper = NewPaper {
        arxiv_id: paper.0.arxiv_id,
        title: paper.0.title,
        description: paper.0.description,
        submission_date: paper.0.submission_date,
        body: paper.0.body,
    };

    // TODO: handle case if paper already exists
    db.insert_paper_full(new_paper, paper.0.authors, paper.0.subjects)
        .await
        .unwrap();

    actix_web::HttpResponse::Created()
}
