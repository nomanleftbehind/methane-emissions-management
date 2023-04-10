use super::super::UUID;
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/mutations.graphql",
    response_derives = "Debug, Clone, PartialEq"
)]
pub struct InsertController;
