use super::super::{NaiveDateTime, UUID};
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.json",
    query_path = "./graphql/queries.graphql",
    response_derives = "Debug, Clone, PartialEq"
)]
#[derive(Debug)]
pub struct GetControllers;
