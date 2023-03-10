use super::super::UUID;
use graphql_client::GraphQLQuery;
use common::FacilityType;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.json",
    query_path = "./graphql/queries.graphql",
    response_derives = "Debug, Clone, PartialEq",
    extern_enums("FacilityType")
)]
#[derive(Debug)]
pub struct AllFacilities;
