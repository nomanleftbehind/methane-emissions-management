use async_graphql::EmptySubscription;

pub mod mutations;
pub mod queries;

pub mod context;
pub mod dataloaders;
pub mod domain;
pub mod sql;

pub use mutations::FullMutation;
pub use queries::FullQuery;

pub type Schema = async_graphql::Schema<FullQuery, FullMutation, EmptySubscription>;
