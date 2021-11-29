use serde::{Deserialize, Serialize};

#[derive(std::fmt::Debug, juniper::GraphQLObject, Serialize, Deserialize)]
pub struct BlogHeader {
    pub slug: String,
    title: String,
    emoji: String,
    posted: chrono::DateTime<chrono::Utc>,
    tags: Vec<String>,
}

#[derive(std::fmt::Debug, juniper::GraphQLObject, Serialize, Deserialize)]
pub struct Blog {
    header: BlogHeader,
    body: String
}

impl Blog {
    pub fn new(header: BlogHeader, body: String) -> Self {
        Self { header, body }
    }
}

