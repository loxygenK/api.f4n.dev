use serde::{Deserialize, Serialize};

#[derive(juniper::GraphQLObject, Serialize, Deserialize)]
pub struct Blog {
    title: String,
    emoji: String,
    posted: chrono::DateTime<chrono::Utc>,
    tags: Vec<String>,
    content: String
}

