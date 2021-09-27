use crate::domain::{basic::Basic, career::Career, contact::Contact, skill::Skill, work::Work};

use super::{Repository, RepositoryResult, util::serialize_yaml};

struct AssetRepository;

impl Repository for AssetRepository {
    fn fetch_basic() -> RepositoryResult<Basic> {
        serialize_yaml("asset/basic.yaml")
    }

    fn fetch_careers() -> RepositoryResult<Vec<Career>> {
        serialize_yaml("asset/career.yaml")
    }

    fn fetch_contacts() -> RepositoryResult<Vec<Contact>> {
        serialize_yaml("asset/content.yaml")
    }

    fn fetch_skills() -> RepositoryResult<Vec<Skill>> {
        serialize_yaml("asset/skill.yaml")
    }

    fn fetch_works() -> RepositoryResult<Vec<Work>> {
        serialize_yaml("asset/work.yaml")
    }
}
