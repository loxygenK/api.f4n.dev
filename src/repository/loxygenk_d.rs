use serde::de::DeserializeOwned;

use crate::domain::{
    basic::Basic,
    blog::{Blog, BlogHeader},
    career::Career,
    contact::Contact,
    skill::Skill,
    work::Work
};

use super::{Repository, RepositoryError, RepositoryResult};

const LOXYGENK_D_PATH: &str = "https://raw.githubusercontent.com/loxygenK/loxygenk.d/feat/new-format";

pub struct LoxygenKDRepository;
impl Repository for LoxygenKDRepository {
    fn fetch_basic(&self) -> super::RepositoryResult<Basic> {
        retrieve_from("basic.yaml")
    }

    fn fetch_blog(&self) -> super::RepositoryResult<Vec<BlogHeader>> {
       retrieve_from("blog/registry.yaml")
    }

    fn fetch_blog_body(&self, slug: &str) -> RepositoryResult<Option<Blog>> {
        let blog_headers: Vec<BlogHeader> = retrieve_from("blog/registry.yaml")?;
        let header = blog_headers
            .into_iter()
            .find(|h| h.slug == slug);

        match header {
            Some(h) => Ok(Some(Blog::new(
                h, retrieve(&format!("blog/{}.md", slug))?
            ))),
            None => Ok(None)
        }
    }

    fn fetch_careers(&self) -> super::RepositoryResult<Vec<Career>> {
        retrieve_from("career.yaml")
    }

    fn fetch_contacts(&self) -> super::RepositoryResult<Vec<Contact>> {
        retrieve_from("contacts.yaml")
    }

    fn fetch_skills(&self) -> super::RepositoryResult<Vec<Skill>> {
        retrieve_from("skills.yaml")
    }

    fn fetch_works(&self) -> super::RepositoryResult<Vec<Work>> {
        retrieve_from("works.yaml")
    }
}

fn retrieve(path: &str) -> RepositoryResult<String> {
    Ok(
        ureq::get(&format!("{}/{}", LOXYGENK_D_PATH, path))
            .call()
            .map_err(|e| RepositoryError::RetrievingError(Box::new(e)))?
            .into_string()
            .map_err(|e| RepositoryError::RetrievingError(Box::new(e)))?
    )
}

fn retrieve_from<T: DeserializeOwned>(path: &str) -> RepositoryResult<T> {
    let content = retrieve(path)?;
    serde_yaml::from_str(&content).map_err(|e| RepositoryError::DeserializationError(e))
}
