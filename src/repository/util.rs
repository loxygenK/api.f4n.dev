use std::{fs::File, io::BufReader};

use serde::de::DeserializeOwned;

use super::{RepositoryError, RepositoryResult};

pub fn serialize_yaml<T: DeserializeOwned>(file_name: &str) -> RepositoryResult<T> {
    let file = File::open(file_name)
        .map_err(|e| RepositoryError::RetrievingError(e))?;

    let reader = BufReader::new(file);

    let result = serde_yaml::from_reader(reader)
        .map_err(|e| RepositoryError::DeserializationError(e))?;

    Ok(result)
}

