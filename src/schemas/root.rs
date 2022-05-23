use crate::db::DbPool;
use actix_web::web::Data;
use juniper::{EmptySubscription, RootNode};

pub struct Query;
pub struct Mutation;

// #[derive(Clone)]
pub struct Context {
    pub db_pool: Data<DbPool>,
}

impl juniper::Context for Context {}

pub type SchemaGraphQL = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> SchemaGraphQL {
    SchemaGraphQL::new(Query {}, Mutation {}, EmptySubscription::new())
}

pub fn create_context(db_pool: Data<DbPool>) -> Context {
    Context {
        db_pool,
    }
}
