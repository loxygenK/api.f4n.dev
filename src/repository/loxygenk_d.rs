use serde::de::DeserializeOwned;

use crate::domain::{
    basic::Basic,
    career::Career,
    contact::Contact,
    skill::Skill,
    work::Work
};

use super::{Repository, RepositoryError, RepositoryResult};

const LOXYGENK_D_PATH: &str = "https://raw.githubusercontent.com/loxygenK/loxygenk.d/main";

pub struct LoxygenKDRepository;
impl Repository for LoxygenKDRepository {
    fn fetch_basic(&self) -> super::RepositoryResult<Basic> {
        todo!()
    }

    fn fetch_careers(&self) -> super::RepositoryResult<Vec<Career>> {
        todo!()
    }

    fn fetch_contacts(&self) -> super::RepositoryResult<Vec<Contact>> {
        todo!()
    }

    fn fetch_skills(&self) -> super::RepositoryResult<Vec<Skill>> {
        todo!()
    }

    fn fetch_works(&self) -> super::RepositoryResult<Vec<Work>> {
        todo!()
    }
}

fn retrieve(path: &str) -> RepositoryResult<String> {
    reqwest::blocking::get(format!("{}/{}", LOXYGENK_D_PATH, path))
        .map_err(|e| RepositoryError::RetrievingError(Box::new(e)))?
        .text()
        .map_err(|e| RepositoryError::RetrievingError(Box::new(e)))
}

fn serialize_yaml<T: DeserializeOwned>(content: &str) -> RepositoryResult<T> {
    serde_yaml::from_str(content).map_err(|e| RepositoryError::DeserializationError(e))
}
