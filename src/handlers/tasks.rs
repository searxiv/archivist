use crate::{db, models};
use actix_web::{
    body::BoxBody,
    get,
    http::header::ContentType,
    post,
    web::{Data, Json, Path},
    HttpResponse, Responder, Result,
};

#[utoipa::path(
    responses(
        (status = 200, description = "Get next task", body = NewTask),
        (status = 404, description = "No tasks available")
    )
)]
#[get("/tasks")]
pub async fn get_task(db: Data<db::DBConnection>) -> Result<HttpResponse<BoxBody>> {
    match db.get_next_task().await? {
        Some(task) => {
            let task = models::NewTask {
                submission_date: task.submission_date,
            };
            let task = serde_json::to_string(&task)?;

            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(task))
        }
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get info about task queue", body = TasksStats)
    )
)]
#[get("/tasks/status")]
pub async fn get_status(db: Data<db::DBConnection>) -> Result<impl Responder> {
    Ok(Json(db.get_tasks_stats().await?))
}

#[utoipa::path(
    responses(
        (status = 201, description = "Create new task to scrape specific day"),
        (status = 400, description = "Invalid date")
    ),
    params(
        ("year",),
        ("month",),
        ("day",)
    ),
)]
#[post("/tasks/{year}/{month}/{day}")]
pub async fn post_day_as_task(
    db: Data<db::DBConnection>,
    date: Path<(i32, u32, u32)>,
) -> Result<HttpResponse<BoxBody>> {
    let (year, month, day) = date.into_inner();

    // TODO: return 400 on incorrect date
    let submission_date = chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let new_task = models::NewTask { submission_date };

    db.insert_task(new_task).await?;

    Ok(HttpResponse::Created().finish())
}
