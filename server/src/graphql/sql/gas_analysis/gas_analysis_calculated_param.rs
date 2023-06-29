use crate::graphql::models::{
    gas_analysis::{
        GasAnalysisCalculatedParamInterim, GasAnalysisCalculatedParamInterimNestedRows,
        GasAnalysisCalculatedParamInterimUnnestedRows,
    },
    input::MonthRangeInput,
};
use sqlx::{query_file, query_file_as, Error, PgPool};
use uuid::Uuid;

pub async fn insert_gas_analysis_calculated_param(
    pool: &PgPool,
    user_id: Uuid,
    MonthRangeInput {
        from_month,
        to_month,
    }: &MonthRangeInput,
) -> Result<u64, Error> {
    let gas_analysis_calculated_param_interim = query_file_as!(
        GasAnalysisCalculatedParamInterim,
        "src/graphql/sql/statements/gas_analysis_calculated_param_calculate.sql",
        from_month,
        to_month
    )
    .fetch_all(pool)
    .await?;

    let GasAnalysisCalculatedParamInterimNestedRows {
        id,
        gas_gravity,
        higher_heating_value,
        carbon_content,
        created_at,
    } = GasAnalysisCalculatedParamInterimUnnestedRows(gas_analysis_calculated_param_interim).into();

    let rows_inserted = query_file!(
        "src/graphql/sql/statements/gas_analysis_calculated_param_insert.sql",
        &id,
        &gas_gravity,
        &higher_heating_value,
        &carbon_content,
        &created_at,
        user_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_inserted)
}
