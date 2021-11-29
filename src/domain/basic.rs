use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(GraphQLObject, Serialize, Deserialize)]
pub struct Basic {
    name: Name,
    introduction: String,
    affiliation: Vec<Affiliation>,
    age: i32,
}

#[derive(GraphQLObject, Serialize, Deserialize)]
pub struct Name {
    primary: String,
    aka: Vec<String>,
}

#[derive(GraphQLObject, Serialize, Deserialize)]
pub struct Affiliation {
    location: String,
    assign: String,
}
