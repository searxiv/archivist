use crate::{
    db,
    models::{self, TaskSubmission},
};
use actix_multipart::Multipart;
use actix_web::{
    get,
    http::header::ContentType,
    post, put,
    web::{Data, Path},
    HttpResponse, Result,
};
use chrono::Datelike;

#[utoipa::path(
    responses(
        (status = 200, description = "Get next task", body = NewTask),
        (status = 404, description = "No tasks available")
    )
)]
#[get("/tasks")]
pub async fn get_task(db: Data<db::DBConnection>) -> Result<HttpResponse> {
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
        None => Ok(HttpResponse::NotFound().body("No tasks available")),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get info about task queue", body = TasksStats)
    )
)]
#[get("/tasks/stats")]
pub async fn get_stats(db: Data<db::DBConnection>) -> Result<HttpResponse> {
    let stats = db.get_tasks_stats().await?;

    Ok(HttpResponse::Ok().json(stats))
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
) -> Result<HttpResponse> {
    let (year, month, day) = date.into_inner();

    let Some(submission_date) = chrono::NaiveDate::from_ymd_opt(year, month, day) else {
        return Ok(HttpResponse::BadRequest().into());
    };
    let new_task = models::NewTask { submission_date };

    db.insert_task(vec![new_task]).await?;

    Ok(HttpResponse::Created().into())
}

#[utoipa::path(
    responses(
        (status = 201, description = "Create new tasks to scrape specific month"),
        (status = 400, description = "Invalid date")
    ),
    params(
        ("year",),
        ("month",),
    ),
)]
#[post("/tasks/{year}/{month}")]
pub async fn post_month_as_task(
    db: Data<db::DBConnection>,
    date: Path<(i32, u32)>,
) -> Result<HttpResponse> {
    let (year, month) = date.into_inner();

    let mut tasks = Vec::new();

    let Some(iter_days) = chrono::NaiveDate::from_ymd_opt(year, month, 1) else {
        return Ok(HttpResponse::BadRequest().body("Invalid date"));
    };

    for day in iter_days.iter_days() {
        if day.month() != month {
            break;
        }
        let new_task = models::NewTask {
            submission_date: day,
        };
        tasks.push(new_task);
    }

    db.insert_task(tasks).await?;

    Ok(HttpResponse::Created().into())
}

#[utoipa::path(
    responses(
        (status = 201, description = "Create new tasks to scrape specific year"),
        (status = 400, description = "Invalid date")
    ),
    params(
        ("year",),
    ),
)]
#[post("/tasks/{year}")]
pub async fn post_year_as_task(
    db: Data<db::DBConnection>,
    date: Path<i32>,
) -> Result<HttpResponse> {
    let year = date.into_inner();

    let mut tasks = Vec::new();

    let Some(iter_days) = chrono::NaiveDate::from_ymd_opt(year, 1, 1) else {
        return Ok(HttpResponse::BadRequest().body("Invalid date"));
    };

    for day in iter_days.iter_days() {
        if day.year() != year {
            break;
        }
        let new_task = models::NewTask {
            submission_date: day,
        };
        tasks.push(new_task);
    }

    db.insert_task(tasks).await?;

    Ok(HttpResponse::Created().into())
}

#[utoipa::path(
    request_body = TaskSubmission,
    responses(
        (status = 201, description = "Task submitted successfully"),
        (status = 400, description = "Invalid task result")
    ),
    params(
        ("submission_date",),
    ),
)]
#[put("/tasks/{submission_date}")]
pub async fn submit_task(
    db: Data<db::DBConnection>,
    path: Path<String>,
    payload: Multipart,
) -> Result<HttpResponse> {
    let submission_date = path.into_inner();
    crate::file_upload::save_file(payload, submission_date.clone())
        .await
        .unwrap();

    let mut storage = crate::file_upload::UPLOAD_STORAGE.lock().await;
    let Some(data) = storage.remove(&submission_date) else {
        return Ok(HttpResponse::InternalServerError().body("Upload not found in storage"));
    };
    let submission: TaskSubmission = serde_json::from_slice(&data)?;

    db.submit_task(submission).await?;

    Ok(HttpResponse::Created().into())
}
