use juniper::{graphql_object, FieldResult};

use crate::domain::basic::{Affiliation, Basic, Name};

use super::QueryRoot;

#[graphql_object]
impl QueryRoot {
    fn basic() -> FieldResult<Basic> {
        Ok(Basic::new(
            Name::new(
                "loxygen.k".to_string(),
                vec!["Flisan".to_string()]
            ),
            "Hello, I'm Flisan!".to_string(),
            vec![
                Affiliation::new(
                    "NITIC".to_string(),
                    "3rd grade, Information dep.".to_string()
                )
            ],
            18
        ))
    }
}
