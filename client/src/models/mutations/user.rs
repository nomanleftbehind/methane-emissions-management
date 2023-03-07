use super::super::{Role, UUID};
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/mutations.graphql",
    response_derives = "Debug, Clone, PartialEq",
    extern_enums("Role")
)]
pub struct Login;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/mutations.graphql",
    response_derives = "Debug, Clone, PartialEq"
)]
pub struct Logout;
