use super::super::{NaiveDate, NaiveDateTime, UUID};
use common::PneumaticInstrumentType;
// use common::{PneumaticInstrumentType, PneumaticInstrumentsByVariant};
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "./graphql/schema.json",
    query_path = "./graphql/queries.graphql",
    response_derives = "Debug, Clone, PartialEq",
    extern_enums("PneumaticInstrumentType")
)]
// extern_enums("PneumaticInstrumentsByVariant", "PneumaticInstrumentType")
pub struct GetPneumaticInstruments;
