use crate::{
    db,
    models::{ArchiveStats, NewPaper, NewPaperFull},
};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder, Result,
};

#[utoipa::path(
    responses(
        (status = 200, description = "Get stats about archive", body = ArchiveStats)
    )
)]
#[get("/archive/status")]
pub async fn get_status(db: Data<db::DBConnection>) -> Result<impl Responder> {
    let stats = ArchiveStats {
        count: db.count_papers().await?,
    };

    Ok(Json(stats))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get all paper submitted in this day", body = [Paper]),
        (status = 400, description = "Invalid date")
    ),
    params(
        ("year",),
        ("month",),
        ("day",)
    ),
)]
#[get("/archive/{year}/{month}/{day}")]
pub async fn get_papers_from_day(
    db: Data<db::DBConnection>,
    date: Path<(i32, u32, u32)>,
) -> Result<impl Responder> {
    let (year, month, day) = date.into_inner();
    // TODO: return 400 on incorrect date
    let date = chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap();

    let papers = db.get_papers_by_date(date).await?;

    Ok(Json(papers))
}

#[utoipa::path(
    request_body = NewPaperFull,
    responses(
        (status = 201, description = "Add new paper into archive"),
        (status = 400, description = "Body cannot be deserialized")
    )
)]
#[post("/archive")]
pub async fn post_paper(
    db: Data<db::DBConnection>,
    paper: Json<NewPaperFull>,
) -> Result<impl Responder> {
    let new_paper_full = paper.0;
    let new_paper = NewPaper {
        arxiv_id: new_paper_full.arxiv_id,
        title: new_paper_full.title,
        description: new_paper_full.description,
        submission_date: new_paper_full.submission_date,
        body: new_paper_full.body,
    };

    db.insert_paper_full(new_paper, new_paper_full.authors, new_paper_full.subjects)
        .await?;

    Ok(HttpResponse::Created())
}
