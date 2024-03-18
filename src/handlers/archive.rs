use crate::{db, models::ArchiveStats};
use actix_web::{
    get,
    web::{Data, Json, Path},
    Responder, Result,
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
