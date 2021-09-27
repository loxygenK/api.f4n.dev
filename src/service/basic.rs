use juniper::{graphql_object, FieldResult};

use crate::domain::basic::{Affiliation, Basic, Name};

use super::{state::State, QueryRoot};

#[graphql_object(context = State)]
impl QueryRoot {
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
}
