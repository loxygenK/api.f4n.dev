use juniper::{EmptyMutation, EmptySubscription};

pub struct QueryRoot;

pub mod basic;

pub type GraphQLScheme = juniper::RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn generate_scheme() -> GraphQLScheme {
    GraphQLScheme::new(QueryRoot, EmptyMutation::new(), EmptySubscription::new())
}
