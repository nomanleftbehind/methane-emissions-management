use super::super::UUID;
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/queries.graphql",
    response_derives = "Debug, Clone, PartialEq"
)]
#[derive(Debug)]
pub struct GetControllers;