use crate::repository::controller::resolver::{ControllerMutation, ControllerQuery, Subscription};
use crate::repository::session::model::ActiveSessionQuery;
use crate::repository::user::resolver::{UserMutation, UserQuery};
use async_graphql::{MergedObject, Schema, SchemaBuilder};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, ControllerQuery, ActiveSessionQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, ControllerMutation);

pub type AppSchema = Schema<Query, Mutation, Subscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, Subscription>;
