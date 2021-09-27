#[derive(juniper::GraphQLObject)]
pub struct Career {
    when: chrono::DateTime<chrono::Utc>,
    title: String,
    prize: Option<String>,
    description: Option<String>,
}

impl Career {
    pub fn new(
        when: chrono::DateTime<chrono::Utc>,
        title: String,
        prize: Option<String>,
        description: Option<String>,
    ) -> Self {
        Self {
            when,
            title,
            prize,
            description,
        }
    }
}
