pub type Id = i32;

#[derive(PartialEq, Debug, sqlx::FromRow)]
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

#[derive(Clone, Debug)]
pub struct NewAuthor {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct NewSubject {
    pub name: String,
}
