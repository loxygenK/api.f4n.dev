use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct Basic {
    name: Name,
    introduction: String,
    affiliation: Vec<Affiliation>,
    age: i32
}

#[derive(GraphQLObject)]
pub struct Name {
    primary: String,
    aka: Vec<String>
}

#[derive(GraphQLObject)]
pub struct Affiliation {
    location: String,
    assign: String
}
