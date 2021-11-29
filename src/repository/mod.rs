use crate::domain::{basic::Basic, blog::BlogHeader, career::Career, contact::Contact, skill::Skill, work::Work};

pub mod mock;
pub mod loxygenk_d;
pub mod frontmatter;

pub enum RepositoryError {
    RetrievingError(Box<dyn std::error::Error>),
    DeserializationError(serde_yaml::Error),
}

type RepositoryResult<T> = Result<T, RepositoryError>;

pub trait Repository: Send + Sync {
    fn fetch_basic(&self) -> RepositoryResult<Basic>;
    fn fetch_blog(&self) -> RepositoryResult<Vec<BlogHeader>>;
    fn fetch_careers(&self) -> RepositoryResult<Vec<Career>>;
    fn fetch_contacts(&self) -> RepositoryResult<Vec<Contact>>;
    fn fetch_skills(&self) -> RepositoryResult<Vec<Skill>>;
    fn fetch_works(&self) -> RepositoryResult<Vec<Work>>;
}
