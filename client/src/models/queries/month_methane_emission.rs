use super::super::{NaiveDate, NaiveDateTime, UUID};
use common::{MethaneEmissionCategory, MethaneEmissionSource, MethaneEmissionSourceTable};
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "./graphql/schema.json",
    query_path = "./graphql/queries.graphql",
    variables_derives = "PartialEq, Clone",
    response_derives = "Debug, Clone, PartialEq",
    extern_enums(
        "MethaneEmissionSourceTable",
        "MethaneEmissionSource",
        "MethaneEmissionCategory"
    )
)]
pub struct GetMonthMethaneEmissions;
