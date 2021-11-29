use crate::domain::{
    basic::Basic,
    blog::{Blog, BlogHeader},
    career::Career,
    contact::Contact,
    skill::Skill,
    work::Work
};

use std::{fs::File, io::BufReader, io::Read};

use serde::de::DeserializeOwned;

use super::{Repository, RepositoryError, RepositoryResult};

pub struct AssetRepository;

impl Repository for AssetRepository {
    fn fetch_basic(&self) -> RepositoryResult<Basic> {
        serialize_yaml("asset/basic.yaml")
    }

    fn fetch_blog(&self) -> RepositoryResult<Vec<BlogHeader>> {
        serialize_yaml("asset/blog/registry.yaml")
    }

    fn fetch_blog_body(&self, slug: &str) -> RepositoryResult<Option<Blog>> {
        let blog_headers: Vec<BlogHeader> = dbg!(serialize_yaml("asset/blog/registry.yaml"))?;

        let header = blog_headers
            .into_iter()
            .find(|h| h.slug == slug);

        match header {
            Some(h) => Ok(Some(Blog::new(
                h, read_file(&format!("asset/blog/{}.md", slug))?
            ))),
            None => Ok(None)
        }
    }

    fn fetch_careers(&self) -> RepositoryResult<Vec<Career>> {
        serialize_yaml("asset/career.yaml")
    }

    fn fetch_contacts(&self) -> RepositoryResult<Vec<Contact>> {
        serialize_yaml("asset/contact.yaml")
    }

    fn fetch_skills(&self) -> RepositoryResult<Vec<Skill>> {
        serialize_yaml("asset/skill.yaml")
    }

    fn fetch_works(&self) -> RepositoryResult<Vec<Work>> {
        serialize_yaml("asset/work.yaml")
    }
}

pub fn read_file(file_name: &str) -> RepositoryResult<String> {
    let file = File::open(file_name)
        .map_err(|e| RepositoryError::RetrievingError(Box::new(e)))?;
    let mut reader = BufReader::new(file);

    let mut text = String::new();
    reader.read_to_string(&mut text)
        .map_err(|e| RepositoryError::RetrievingError(Box::new(e)))?;

    Ok(text)
}

pub fn serialize_yaml<T: DeserializeOwned>(file_name: &str) -> RepositoryResult<T> {
    serde_yaml::from_str(&read_file(file_name)?)
        .map_err(RepositoryError::DeserializationError)
}
