pub type Id = i32;

#[derive(PartialEq, Debug, serde::Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct Paper {
    pub id: Id,
    pub arxiv_id: String,
    pub title: String,
    pub description: String,
    pub submission_date: chrono::NaiveDate,
    pub body: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Author {
    pub id: Id,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Subject {
    pub id: Id,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct PaperAuthor {
    pub paper_id: Id,
    pub author_id: Id,
}

#[derive(Clone, Debug)]
pub struct PaperSubject {
    pub paper_id: Id,
    pub subject_id: Id,
}

#[derive(Clone, Debug)]
pub struct NewPaper {
    pub arxiv_id: String,
    pub title: String,
    pub description: String,
    pub submission_date: chrono::NaiveDate,
    pub body: String,
}

#[derive(Clone, Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct NewPaperFull {
    pub arxiv_id: String,
    pub title: String,
    pub description: String,
    pub submission_date: chrono::NaiveDate,
    pub body: String,
    pub authors: Vec<NewAuthor>,
    pub subjects: Vec<NewSubject>,
}

#[derive(Clone, Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct NewAuthor {
    pub name: String,
}

#[derive(Clone, Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct NewSubject {
    pub name: String,
}

#[derive(Clone, Debug, serde::Serialize, utoipa::ToSchema)]
pub struct ArchiveStats {
    pub count: i64,
}
