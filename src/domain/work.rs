use serde::{Deserialize, Serialize};

#[derive(juniper::GraphQLEnum, Serialize, Deserialize)]
pub enum Status {
    Advancing,
    Archived,
    Maintenancing,
    Completed,
    Developing,
}

#[derive(juniper::GraphQLObject, Serialize, Deserialize)]
pub struct Work {
    name: String,
    description: String,
    repo_url: Option<String>,
    image_url: Option<String>,
    tags: Vec<String>,
    status: Status,
}
