use sqlx::postgres::PgPoolOptions;

use crate::models::{self, NewAuthor, NewPaper, NewPaperFull, NewSubject};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),
}
impl actix_web::error::ResponseError for Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct DBConnection {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DBConnection {
    pub async fn new(db_url: &str) -> Result<DBConnection> {
        Ok(DBConnection {
            pool: PgPoolOptions::new().connect(db_url).await?,
        })
    }

    #[allow(unused)]
    pub async fn get_all_papers(&self) -> Result<Vec<models::Paper>> {
        sqlx::query_as!(models::Paper, "SELECT * FROM papers")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn count_papers(&self) -> Result<i64> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM papers")
            .fetch_one(&self.pool)
            .await?
            .unwrap(); // NOTE(mchernigin): I am sure it returns a number

        Ok(count)
    }

    #[allow(unused)]
    pub async fn get_paper(&self, desired_id: i32) -> Result<models::Paper> {
        sqlx::query_as!(
            models::Paper,
            "SELECT * FROM papers
             WHERE id = $1",
            desired_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.into())
    }

    pub async fn get_papers_by_date(&self, date: chrono::NaiveDate) -> Result<Vec<models::Paper>> {
        sqlx::query_as!(
            models::Paper,
            "SELECT * FROM papers
             WHERE submission_date = $1",
            date
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.into())
    }

    #[allow(unused)]
    pub async fn get_paper_authors(&self, desired_paper_id: i32) -> Result<Vec<models::Author>> {
        sqlx::query_as!(
            models::Author,
            "SELECT authors.id, authors.name
             FROM authors
             JOIN paper_author ON authors.id = paper_author.author_id
             WHERE paper_author.paper_id = $1",
            desired_paper_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.into())
    }

    #[allow(unused)]
    pub async fn paper_exists(&self, desired_arxiv_id: &str) -> Result<bool> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT * FROM papers WHERE arxiv_id = $1)",
            desired_arxiv_id
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap(); // NOTE(mchernigin): I am sure it returns a bool

        Ok(exists)
    }

    pub async fn insert_paper(
        &self,
        new_paper: NewPaper,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<models::Id> {
        log::trace!("DB: inserting new paper {:?}", new_paper.arxiv_id);
        sqlx::query!(
            "INSERT INTO papers (arxiv_id, title, description, submission_date, body)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (arxiv_id) DO NOTHING",
            new_paper.arxiv_id,
            new_paper.title,
            new_paper.description,
            new_paper.submission_date,
            new_paper.body,
        )
        .execute(&mut **tx)
        .await?;

        Ok(sqlx::query_scalar!(
            "SELECT papers.id FROM papers
             WHERE arxiv_id = $1",
            new_paper.arxiv_id
        )
        .fetch_one(&mut **tx)
        .await?)
    }

    pub async fn insert_author(
        &self,
        new_author: NewAuthor,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<models::Id> {
        log::trace!("DB: inserting new author {:?}", new_author);

        sqlx::query!(
            "INSERT INTO authors (name)
             VALUES ($1)
             ON CONFLICT(name) DO NOTHING",
            new_author.name
        )
        .execute(&mut **tx)
        .await?;

        Ok(sqlx::query_scalar!(
            "SELECT authors.id FROM authors
             WHERE name = $1",
            new_author.name
        )
        .fetch_one(&mut **tx)
        .await?)
    }

    pub async fn insert_subject(
        &self,
        new_subject: NewSubject,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<models::Id> {
        log::trace!("DB: inserting new subject {:?}", new_subject.name);

        sqlx::query!(
            "INSERT INTO subjects (name)
             VALUES ($1)
             ON CONFLICT(name) DO NOTHING",
            new_subject.name
        )
        .execute(&mut **tx)
        .await?;

        Ok(sqlx::query_scalar!(
            "SELECT subjects.id FROM subjects
             WHERE name = $1",
            new_subject.name
        )
        .fetch_one(&mut **tx)
        .await?)
    }

    pub async fn set_paper_author(
        &self,
        paper_id: models::Id,
        author_id: models::Id,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<()> {
        log::trace!(
            "DB: inserting author {:?} for paper {:?}",
            author_id,
            paper_id
        );
        sqlx::query!(
            "INSERT INTO paper_author (paper_id, author_id)
             VALUES ($1, $2)
             ON CONFLICT ON CONSTRAINT paper_author_pkey DO NOTHING",
            paper_id,
            author_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn set_paper_subject(
        &self,
        paper_id: models::Id,
        subject_id: models::Id,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<()> {
        log::trace!(
            "DB: inserting subject {:?} for paper {:?}",
            subject_id,
            paper_id
        );
        sqlx::query!(
            "INSERT INTO paper_subject (paper_id, subject_id)
             VALUES ($1, $2)
             ON CONFLICT ON CONSTRAINT paper_subject_pkey DO NOTHING",
            paper_id,
            subject_id,
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn insert_papers_full(
        &self,
        papers_full: Vec<NewPaperFull>,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<()> {
        for paper_full in papers_full {
            if self.paper_exists(&paper_full.arxiv_id).await? {
                log::warn!(
                    "DB: paper {:?} already exists in archive, skipping",
                    paper_full.arxiv_id
                );
                return Ok(());
            }

            let paper = NewPaper {
                arxiv_id: paper_full.arxiv_id,
                title: paper_full.title,
                description: paper_full.description,
                submission_date: paper_full.submission_date,
                body: paper_full.body,
            };

            let paper_id = self.insert_paper(paper, &mut *tx).await?;

            for author in paper_full.authors {
                let author_id = self.insert_author(author, &mut *tx).await?;
                self.set_paper_author(paper_id, author_id, &mut *tx).await?;
            }

            for subject in paper_full.subjects {
                let subject_id = self.insert_subject(subject, &mut *tx).await?;
                self.set_paper_subject(paper_id, subject_id, &mut *tx)
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn get_next_task(&self) -> Result<Option<models::Task>> {
        let mut tx = self.pool.begin().await?;

        let res = sqlx::query_as!(
            models::Task,
            "SELECT submission_date, status as \"status: _\", processing_start, processing_end
             FROM tasks WHERE status = $1
             LIMIT 1
             FOR UPDATE",
            models::Status::Idle as models::Status,
        )
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(task) = &res {
            log::trace!("DB: given {:?} as next task", task.submission_date);

            sqlx::query!(
                "UPDATE tasks
                 SET status = $1, processing_start = $2
                 WHERE submission_date = $3",
                models::Status::Processing as models::Status,
                chrono::Utc::now().naive_utc(),
                task.submission_date
            )
            .execute(&mut *tx)
            .await?;
        } else {
            log::trace!("DB: next task requested but queue is empty");
        }

        tx.commit().await?;

        Ok(res)
    }

    pub async fn insert_task(&self, new_tasks: Vec<models::NewTask>) -> Result<()> {
        log::trace!("DB: inserting new tasks ({})", new_tasks.len());

        let submission_dates = new_tasks
            .into_iter()
            .map(|models::NewTask { submission_date }| submission_date)
            .collect::<Vec<_>>();
        let statuses = vec![models::Status::Idle; submission_dates.len()];

        sqlx::query!(
            r#"INSERT INTO tasks (submission_date, status)
               SELECT *
               FROM UNNEST ($1::date[], $2::status[])
               ON CONFLICT (submission_date) DO NOTHING"#,
            &submission_dates[..],
            &statuses[..] as &[models::Status],
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_tasks_stats(&self) -> Result<models::TasksStats> {
        let mut tx = self.pool.begin().await?;

        let idle = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM tasks
             WHERE status = $1",
            models::Status::Idle as models::Status
        )
        .fetch_one(&mut *tx)
        .await?
        .unwrap(); // NOTE(mchernigin): I am sure it returns a number

        let processing = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM tasks
             WHERE status = $1",
            models::Status::Processing as models::Status
        )
        .fetch_one(&mut *tx)
        .await?
        .unwrap(); // NOTE(mchernigin): I am sure it returns a number

        let done = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM tasks
             WHERE status = $1",
            models::Status::Done as models::Status
        )
        .fetch_one(&mut *tx)
        .await?
        .unwrap(); // NOTE(mchernigin): I am sure it returns a number

        tx.commit().await?;

        Ok(models::TasksStats {
            idle,
            processing,
            done,
        })
    }

    pub async fn submit_task(&self, submission: models::TaskSubmission) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        self.insert_papers_full(submission.papers, &mut tx).await?;

        sqlx::query!(
            "UPDATE tasks
             SET status = $1, processing_end = $2
             WHERE submission_date = $3",
            models::Status::Done as models::Status,
            chrono::Utc::now().naive_utc(),
            submission.submission_date,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn revert_long_running_tasks(&self, threshold_seconds: u64) -> Result<()> {
        let threshold = std::time::Duration::from_secs(threshold_seconds);
        let threshold = sqlx::postgres::types::PgInterval::try_from(threshold).unwrap();

        // TODO:: Use SELECT FOR UPDATE
        sqlx::query!(
            "UPDATE tasks
             SET status = $1, processing_start = $2
             WHERE status = $3
             AND (current_timestamp - processing_start > $4 OR processing_start IS NULL)",
            models::Status::Idle as models::Status,
            None as Option<chrono::NaiveDateTime>,
            models::Status::Processing as models::Status,
            threshold
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_db_size_mb(&self) -> Result<f64> {
        let size_bytes = sqlx::query_scalar!("SELECT pg_database_size('searxiv')")
            .fetch_one(&self.pool)
            .await?
            .unwrap(); // NOTE(mchernigin): I am sure it returns a number

        let size_mb = size_bytes as f64 / 1024. / 1024.;
        Ok(size_mb)
    }
}
