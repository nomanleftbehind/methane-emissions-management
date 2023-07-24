use super::super::{NaiveDate, NaiveDateTime, UUID};
use common::PneumaticInstrumentType;
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "./graphql/schema.json",
    query_path = "./graphql/queries.graphql",
    response_derives = "Debug, Clone, PartialEq",
    variables_derives = "Debug, PartialEq, Clone",
    extern_enums("PneumaticInstrumentType")
)]
pub struct GetPneumaticInstruments;

#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "./graphql/schema.json",
    query_path = "./graphql/queries.graphql",
    response_derives = "Debug, Clone, PartialEq",
    variables_derives = "Debug, PartialEq, Clone"
)]
pub struct GetPneumaticInstrumentChanges;
