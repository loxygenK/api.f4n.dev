pub mod state;

use juniper::{EmptyMutation, EmptySubscription, FieldResult};

use crate::{domain::{
    basic::{Affiliation, Basic, Name},
    career::Career,
    contact::Contact,
    skill::{Skill, SkillType, SkilledLevel},
    work::{Status, Work},
}, repository::Repository, service::Service};

use self::state::State;

pub type GraphQLScheme =
    juniper::RootNode<'static, Query, EmptyMutation<State>, EmptySubscription<State>>;

pub struct Query {
    service: Service
}

#[juniper::graphql_object(context = State)]
impl Query {
    fn basic(_state: &State) -> FieldResult<Basic> {
        Ok(Basic::new(
            Name::new("loxygen.k".to_string(), vec!["Flisan".to_string()]),
            "Hello, I'm Flisan!".to_string(),
            vec![Affiliation::new(
                "NITIC".to_string(),
                "3rd grade, Information dep.".to_string(),
            )],
            18,
        ))
    }

    fn careers(_state: &State) -> FieldResult<Vec<Career>> {
        Ok(vec![Career::new(
            chrono::Utc::now(),
            "whatever".to_string(),
            None,
            None,
        )])
    }

    fn contacts(_state: &State) -> FieldResult<Vec<Contact>> {
        Ok(vec![Contact::new(
            "Twitter".to_string(),
            "loxygenK".to_string(),
            "https://twitter.com/loxygen_k".to_string(),
        )])
    }

    fn skills(_state: &State) -> FieldResult<Vec<Skill>> {
        Ok(vec![Skill::new(
            "Something".to_string(),
            SkillType::Miscellaneous,
            SkilledLevel::Advanced,
            "whatever".to_string(),
        )])
    }

    fn works(_state: &State) -> FieldResult<Vec<Work>> {
        Ok(vec![Work::new(
            "whatever".to_string(),
            "hoge".to_string(),
            Some("repo".to_string()),
            None,
            vec!["what".to_string(), "ever".to_string()],
            Status::Advancing,
        )])
    }
}

pub fn generate_scheme(service: Service) -> GraphQLScheme {
    GraphQLScheme::new(
        Query { service },
        EmptyMutation::<State>::new(),
        EmptySubscription::<State>::new(),
    )
}

