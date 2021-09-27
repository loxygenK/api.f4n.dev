use crate::domain::{basic::Basic, career::Career, contact::Contact, skill::Skill, work::Work};

use super::{Repository, RepositoryResult};

struct AssetRepository;

impl Repository for AssetRepository {
    fn fetch_basic() -> RepositoryResult<Basic> {
        todo!()
    }

    fn fetch_careers() -> RepositoryResult<Vec<Career>> {
        todo!()
    }

    fn fetch_contacts() -> RepositoryResult<Vec<Contact>> {
        todo!()
    }

    fn fetch_skills() -> RepositoryResult<Vec<Skill>> {
        todo!()
    }

    fn fetch_works() -> RepositoryResult<Vec<Work>> {
        todo!()
    }
}
