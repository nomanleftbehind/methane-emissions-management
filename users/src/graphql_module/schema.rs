use super::modules::controller_model::resolver::{PostMutation, PostQuery, Subscription};
use super::modules::user_model::resolver::{AuthUser, UserMutate};
use async_graphql::{MergedObject, Schema, SchemaBuilder};

#[derive(MergedObject, Default)]
pub struct Query(AuthUser, PostQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutate, PostMutation);

pub type AppSchema = Schema<Query, Mutation, Subscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, Subscription>;
