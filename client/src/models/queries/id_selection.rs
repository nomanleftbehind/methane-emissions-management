// use super::super::UUID;
// use common::IdSelectionVariant;
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery, Debug, PartialEq, Clone)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries.graphql",
    variables_derives = "PartialEq, Clone",
    response_derives = "Debug, Clone, PartialEq"
)]
// extern_enums("IdSelectionVariant")
pub struct IdSelection;
