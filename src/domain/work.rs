#[derive(juniper::GraphQLEnum)]
pub enum Status {
    Advancing,
    Maintenancing,
    Completed,
    Developing,
}

#[derive(juniper::GraphQLObject)]
pub struct Work {
    name: String,
    description: String,
    repo_url: Option<String>,
    image_url: Option<String>,
    tags: Vec<String>,
    status: Status,
}

impl Work {
    pub fn new(
        name: String,
        description: String,
        repo_url: Option<String>,
        image_url: Option<String>,
        tags: Vec<String>,
        status: Status,
    ) -> Self {
        Self {
            name,
            description,
            repo_url,
            image_url,
            tags,
            status,
        }
    }
}
