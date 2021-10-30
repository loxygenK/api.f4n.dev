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
        retrieve_from("basic.yaml")
    }

    fn fetch_careers(&self) -> super::RepositoryResult<Vec<Career>> {
        retrieve_from("career.yaml")
    }

    fn fetch_contacts(&self) -> super::RepositoryResult<Vec<Contact>> {
        retrieve_from("contacts.yaml")
    }

    fn fetch_skills(&self) -> super::RepositoryResult<Vec<Skill>> {
        retrieve_from("skills.yaml")
    }

    fn fetch_works(&self) -> super::RepositoryResult<Vec<Work>> {
        retrieve_from("works.yaml")
    }
}

fn retrieve_from<T: DeserializeOwned>(path: &str) -> RepositoryResult<T> {
    let content = reqwest::blocking::get(format!("{}/{}", LOXYGENK_D_PATH, path))
        .map_err(|e| RepositoryError::RetrievingError(Box::new(e)))?
        .text()
        .map_err(|e| RepositoryError::RetrievingError(Box::new(e)))?;

    serde_yaml::from_str(&content).map_err(|e| RepositoryError::DeserializationError(e))
}
