pub mod state;

use juniper::{EmptyMutation, EmptySubscription, FieldError, FieldResult};

use crate::{
    domain::{
        basic::{Affiliation, Basic, Name},
        career::Career,
        contact::Contact,
        skill::{Skill, SkillType, SkilledLevel},
        work::{Status, Work},
    },
    service::Service,
};

use self::state::State;

pub type GraphQLScheme =
    juniper::RootNode<'static, Query, EmptyMutation<State>, EmptySubscription<State>>;

pub struct Query {
    service: Service,
}

#[juniper::graphql_object(context = State)]
impl Query {
    fn basic(&self, _state: &State) -> FieldResult<Basic> {
        self.service
            .fetch_basic()
            .map_err(|e| FieldError::new(e, juniper::Value::Null))
    }

    fn careers(&self, _state: &State) -> FieldResult<Vec<Career>> {
        self.service
            .fetch_careers()
            .map_err(|e| FieldError::new(e, juniper::Value::Null))
    }

    fn contacts(&self, _state: &State) -> FieldResult<Vec<Contact>> {
        self.service
            .fetch_contacts()
            .map_err(|e| FieldError::new(e, juniper::Value::Null))
    }

    fn skills(&self, _state: &State) -> FieldResult<Vec<Skill>> {
        self.service
            .fetch_skills()
            .map_err(|e| FieldError::new(e, juniper::Value::Null))
    }

    fn works(&self, _state: &State) -> FieldResult<Vec<Work>> {
        self.service
            .fetch_works()
            .map_err(|e| FieldError::new(e, juniper::Value::Null))
    }
}

impl Query {
    pub fn new(service: Service) -> Self {
        Self { service }
    }

    pub fn generate_scheme(self) -> GraphQLScheme {
        GraphQLScheme::new(
            self,
            EmptyMutation::<State>::new(),
            EmptySubscription::<State>::new(),
        )
    }
}
