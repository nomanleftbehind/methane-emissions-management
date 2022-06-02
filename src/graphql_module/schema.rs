use super::models::controller_model::resolver::{ControllerMutation, ControllerQuery, Subscription};
use super::models::user_model::resolver::{UserQuery, UserMutation};
use async_graphql::{MergedObject, Schema, SchemaBuilder};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, ControllerQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, ControllerMutation);

pub type AppSchema = Schema<Query, Mutation, Subscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, Subscription>;
