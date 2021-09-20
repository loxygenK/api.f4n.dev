use juniper::{EmptyMutation, EmptySubscription};

use self::state::State;

pub struct QueryRoot;

pub mod basic;
pub mod state;

pub type GraphQLScheme = juniper::RootNode<'static, QueryRoot, EmptyMutation<State>, EmptySubscription<State>>;

pub fn generate_scheme() -> GraphQLScheme {
    GraphQLScheme::new(QueryRoot, EmptyMutation::<State>::new(), EmptySubscription::<State>::new())
}
