use serde::{Serialize, Deserialize};

#[derive(juniper::GraphQLObject, Serialize, Deserialize)]
pub struct Contact {
    service: String,
    identifier: String,
    url: String,
}

impl Contact {
    pub fn new(service: String, identifier: String, url: String) -> Self {
        Self {
            service,
            identifier,
            url,
        }
    }
}
