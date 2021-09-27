use crate::domain::{basic::Basic, career::Career, contact::Contact, skill::Skill, work::Work};

use std::{fs::File, io::BufReader};

use serde::de::DeserializeOwned;

use super::{Repository, RepositoryError, RepositoryResult};

struct AssetRepository;

impl Repository for AssetRepository {
    fn fetch_basic() -> RepositoryResult<Basic> {
        serialize_yaml("asset/basic.yaml")
    }

    fn fetch_careers() -> RepositoryResult<Vec<Career>> {
        serialize_yaml("asset/career.yaml")
    }

    fn fetch_contacts() -> RepositoryResult<Vec<Contact>> {
        serialize_yaml("asset/content.yaml")
    }

    fn fetch_skills() -> RepositoryResult<Vec<Skill>> {
        serialize_yaml("asset/skill.yaml")
    }

    fn fetch_works() -> RepositoryResult<Vec<Work>> {
        serialize_yaml("asset/work.yaml")
    }
}

pub fn serialize_yaml<T: DeserializeOwned>(file_name: &str) -> RepositoryResult<T> {
    let file = File::open(file_name)
        .map_err(|e| RepositoryError::RetrievingError(e))?;

    let reader = BufReader::new(file);

    let result = serde_yaml::from_reader(reader)
        .map_err(|e| RepositoryError::DeserializationError(e))?;

    Ok(result)
}
