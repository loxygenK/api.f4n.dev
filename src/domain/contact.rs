use serde::{Deserialize, Serialize};

#[derive(juniper::GraphQLObject, Serialize, Deserialize)]
pub struct Contact {
    service: String,
    identifier: String,
    url: String,
}


