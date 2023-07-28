use super::super::{NaiveDate, UUID};
use common::PneumaticInstrumentType;
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery, Debug, PartialEq, Clone)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/mutations.graphql",
    variables_derives = "PartialEq, Clone",
    response_derives = "Debug, Clone, PartialEq",
    extern_enums("PneumaticInstrumentType")
)]
pub struct InsertPneumaticInstrument;

#[derive(GraphQLQuery, Debug, PartialEq, Clone)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/mutations.graphql",
    variables_derives = "PartialEq, Clone",
    response_derives = "Debug, Clone, PartialEq"
)]
pub struct InsertPneumaticInstrumentEmissionRate;
