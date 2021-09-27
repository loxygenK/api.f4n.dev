use std::fmt::Display;

use crate::{domain::{basic::Basic, career::Career, contact::Contact, skill::Skill, work::Work}, repository::{Repository, RepositoryError}};

impl Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                RepositoryError::RetrievingError(_) => "Failed to retrievate data!",
                RepositoryError::DeserializationError(_) => "Database contains invalid data!",
            }
        )
    }
}

pub struct Service {
    repository: dyn Repository
}

type ServiceResult<T> = Result<T, String>;

impl Service {
    pub fn fetch_basic(&self) -> ServiceResult<Basic> {
        self.repository.fetch_basic()
            .map_err(|e| e.to_string())
    }

    pub fn fetch_careers(&self) -> ServiceResult<Vec<Career>> {
        self.repository.fetch_careers()
            .map_err(|e| e.to_string())
    }

    pub fn fetch_contacts(&self) -> ServiceResult<Vec<Contact>> {
        self.repository.fetch_contacts()
            .map_err(|e| e.to_string())
    }

    pub fn fetch_skills(&self) -> ServiceResult<Vec<Skill>> {
        self.repository.fetch_skills()
            .map_err(|e| e.to_string())
    }

    pub fn fetch_works(&self) -> ServiceResult<Vec<Work>> {
        self.repository.fetch_works()
            .map_err(|e| e.to_string())
    }
}
