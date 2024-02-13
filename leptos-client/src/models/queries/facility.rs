use super::super::UUID;
use common::FacilityType;
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "./graphql/schema.json",
    query_path = "./graphql/queries.graphql",
    response_derives = "Debug, Clone, PartialEq, Deserialize, Serialize",
    extern_enums("FacilityType")
)]
pub struct AllFacilities;
