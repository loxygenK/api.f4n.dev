pub mod state;

use self::state::State;
use crate::{
    domain::{basic::Basic, blog::BlogHeader, career::Career, contact::Contact, skill::Skill, work::Work},
    service::Service,
};
use juniper::{EmptyMutation, EmptySubscription, FieldError, FieldResult};

pub type GraphQLScheme =
    juniper::RootNode<'static, Query, EmptyMutation<State>, EmptySubscription<State>>;

pub struct Query {
    service: Service,
}

#[juniper::graphql_object(context = State)]
impl Query {
    fn basic(&self, _state: &State) -> FieldResult<Basic> {
        self.service.fetch_basic().map_err(Query::to_field_error)
    }

    fn blog_property(&self, _state: &State) -> FieldResult<Vec<BlogHeader>> {
        self.service.fetch_blog().map_err(Query::to_field_error)
    }

    fn careers(&self, _state: &State) -> FieldResult<Vec<Career>> {
        self.service.fetch_careers().map_err(Query::to_field_error)
    }

    fn contacts(&self, _state: &State) -> FieldResult<Vec<Contact>> {
        self.service.fetch_contacts().map_err(Query::to_field_error)
    }

    fn skills(&self, _state: &State) -> FieldResult<Vec<Skill>> {
        self.service.fetch_skills().map_err(Query::to_field_error)
    }

    fn works(&self, _state: &State) -> FieldResult<Vec<Work>> {
        self.service.fetch_works().map_err(Query::to_field_error)
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

    fn to_field_error(message: String) -> FieldError {
        FieldError::new(message, juniper::Value::Null)
    }
}
