use crate::domain::{basic::Basic, career::Career, contact::Contact, skill::Skill, work::Work};

pub mod mock;
mod util;

pub enum RepositoryError {
    RetrievingError(std::io::Error),
    DeserializationError(serde_yaml::Error),
}

type RepositoryResult<T> = Result<T, RepositoryError>;

pub trait Repository {
    fn fetch_basic() -> RepositoryResult<Basic>;
    fn fetch_careers() -> RepositoryResult<Vec<Career>>;
    fn fetch_contacts() -> RepositoryResult<Vec<Contact>>;
    fn fetch_skills() -> RepositoryResult<Vec<Skill>>;
    fn fetch_works() -> RepositoryResult<Vec<Work>>;
}
