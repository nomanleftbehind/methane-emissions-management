use super::super::UUID;
use common::Role;
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.json",
    query_path = "./graphql/queries.graphql",
    response_derives = "Debug, Clone, PartialEq, Eq",
    extern_enums("Role")
)]
#[derive(Debug, PartialEq, Eq)]
pub struct Me;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.json",
    query_path = "./graphql/queries.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize",
    extern_enums("Role")
)]
#[derive(Debug, PartialEq)]
pub struct GetUsers;
