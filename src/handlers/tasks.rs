use crate::{db, models};
use actix_web::{
    get,
    http::header::ContentType,
    post,
    web::{Data, Json, Path},
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
    request_body = TaskSubmission,
    responses(
        (status = 201, description = "Task submitted successfully"),
        (status = 400, description = "Invalid task result")
    )
)]
#[post("/tasks")]
pub async fn submit_task(
    db: Data<db::DBConnection>,
    submission: Json<models::TaskSubmission>,
) -> Result<HttpResponse> {
    db.submit_task(submission.0).await?;

    Ok(HttpResponse::Created().into())
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

    let submission_date = match chrono::NaiveDate::from_ymd_opt(year, month, day) {
        Some(d) => d,
        None => {
            return Ok(HttpResponse::BadRequest().into());
        }
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

    let iter_days = match chrono::NaiveDate::from_ymd_opt(year, month, 1) {
        Some(d) => d.iter_days(),
        None => {
            return Ok(HttpResponse::BadRequest().body("Invalid date"));
        }
    };

    for day in iter_days {
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

    let iter_days = match chrono::NaiveDate::from_ymd_opt(year, 1, 1) {
        Some(d) => d.iter_days(),
        None => {
            return Ok(HttpResponse::BadRequest().body("Invalid date"));
        }
    };

    for day in iter_days {
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
