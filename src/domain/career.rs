use serde::{Deserialize, Serialize};

#[derive(juniper::GraphQLObject, Serialize, Deserialize)]
pub struct Career {
    when: chrono::DateTime<chrono::Utc>,
    title: String,
    prize: Option<String>,
    description: Option<String>,
}

