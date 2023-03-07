use super::super::UUID;
use common::Role;
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/queries.graphql",
    response_derives = "Debug, Clone, PartialEq",
    extern_enums("Role")
)]
#[derive(Debug)]
pub struct GetUsers;
