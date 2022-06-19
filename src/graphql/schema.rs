use crate::repository::controller::resolver::{ControllerMutation, ControllerQuery, Subscription};
use crate::repository::user::resolver::{UserQuery, UserMutation};
use async_graphql::{MergedObject, Schema, SchemaBuilder};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, ControllerQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, ControllerMutation);

pub type AppSchema = Schema<Query, Mutation, Subscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, Subscription>;
