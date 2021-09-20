use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct Basic {
    name: Name,
    introduction: String,
    affiliation: Vec<Affiliation>,
    age: i32,
}

impl Basic {
    pub fn new(name: Name, introduction: String, affiliation: Vec<Affiliation>, age: i32) -> Self {
        Self {
            name,
            introduction,
            affiliation,
            age,
        }
    }
}

#[derive(GraphQLObject)]
pub struct Name {
    primary: String,
    aka: Vec<String>,
}

impl Name {
    pub fn new(primary: String, aka: Vec<String>) -> Self {
        Self { primary, aka }
    }
}

#[derive(GraphQLObject)]
pub struct Affiliation {
    location: String,
    assign: String,
}

impl Affiliation {
    pub fn new(location: String, assign: String) -> Self {
        Self { location, assign }
    }
}
