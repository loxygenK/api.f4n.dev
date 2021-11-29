use std::fmt::Display;

use crate::{
    domain::{
        basic::Basic,
        blog::{Blog, BlogHeader},
        career::Career,
        contact::Contact,
        skill::Skill,
        work::Work
    },
    repository::{Repository, RepositoryError},
};

impl Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_msg = match self {
            RepositoryError::RetrievingError(e)
                => format!("Failed to retrievate data!: {}", e.to_string()),
            RepositoryError::DeserializationError(e)
                => format!("Database contains invalid data!: {}", e.to_string()),
        };

        f.write_str(&error_msg)
    }
}

pub struct Service {
    repository: Box<dyn Repository>,
}

type ServiceResult<T> = Result<T, String>;

impl Service {
    pub fn new(repository: Box<dyn Repository>) -> Self {
        Self { repository }
    }

    pub fn fetch_basic(&self) -> ServiceResult<Basic> {
        self.repository.fetch_basic().map_err(|e| e.to_string())
    }

    pub fn fetch_blog(&self) -> ServiceResult<Vec<BlogHeader>> {
        self.repository.fetch_blog().map_err(|e| e.to_string())
    }

    pub fn fetch_blog_body(&self, slug: &str) -> ServiceResult<Blog> {
        let blog = self.repository.fetch_blog_body(slug).map_err(|e| e.to_string())?;

        match blog {
            Some(b) => Ok(b),
            None => Err("Blog with provided slug is not found.".to_string())
        }
    }

    pub fn fetch_careers(&self) -> ServiceResult<Vec<Career>> {
        self.repository.fetch_careers().map_err(|e| e.to_string())
    }

    pub fn fetch_contacts(&self) -> ServiceResult<Vec<Contact>> {
        self.repository.fetch_contacts().map_err(|e| e.to_string())
    }

    pub fn fetch_skills(&self) -> ServiceResult<Vec<Skill>> {
        self.repository.fetch_skills().map_err(|e| e.to_string())
    }

    pub fn fetch_works(&self) -> ServiceResult<Vec<Work>> {
        self.repository.fetch_works().map_err(|e| e.to_string())
    }
}
