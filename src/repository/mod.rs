use crate::domain::{basic::Basic, career::Career, contact::Contact, skill::Skill, work::Work};

pub mod mock;
pub mod loxygenk_d;

pub enum RepositoryError {
    RetrievingError(Box<dyn std::error::Error>),
    DeserializationError(serde_yaml::Error),
}

type RepositoryResult<T> = Result<T, RepositoryError>;

pub trait Repository: Send + Sync {
    fn fetch_basic(&self) -> RepositoryResult<Basic>;
    fn fetch_careers(&self) -> RepositoryResult<Vec<Career>>;
    fn fetch_contacts(&self) -> RepositoryResult<Vec<Contact>>;
    fn fetch_skills(&self) -> RepositoryResult<Vec<Skill>>;
    fn fetch_works(&self) -> RepositoryResult<Vec<Work>>;
}
