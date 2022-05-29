use super::modules;
use super::modules::user_model::resolver::{AuthUser, UserMutate};
use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, SchemaBuilder};
#[derive(MergedObject, Default)]
pub struct Query(AuthUser);
#[derive(MergedObject, Default)]
pub struct Mutation(UserMutate);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;
