use crate::{db, models::ArchiveStats};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Result,
};

#[utoipa::path(
    responses(
        (status = 200, description = "Get stats about archive", body = ArchiveStats)
    )
)]
#[get("/archive/status")]
pub async fn get_status(db: Data<db::DBConnection>) -> Result<HttpResponse> {
    let stats = ArchiveStats {
        paper_count: db.count_papers().await?,
        db_size_mb: db.get_db_size_mb().await?,
    };

    Ok(HttpResponse::Ok().json(stats))
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
) -> Result<HttpResponse> {
    let (year, month, day) = date.into_inner();
    let date = match chrono::NaiveDate::from_ymd_opt(year, month, day) {
        Some(d) => d,
        None => {
            return Ok(HttpResponse::BadRequest().into());
        }
    };

    let papers = db.get_papers_by_date(date).await?;

    Ok(HttpResponse::Ok().json(papers))
}
