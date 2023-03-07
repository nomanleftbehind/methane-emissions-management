use crate::graphql::models::{
    GasAnalysisCalculatedParamInterim, GasAnalysisCalculatedParamInterimNestedRows,
    GasAnalysisCalculatedParamInterimUnnestedRows,
};
use sqlx::{query_file, query_file_as, Error, PgPool};
use uuid::Uuid;

pub async fn insert_gas_analysis_calculated_params(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<u64, Error> {
    let gas_analysis_calculated_params_interim = query_file_as!(
        GasAnalysisCalculatedParamInterim,
        "src/graphql/sql/statements/gas_analysis_calculated_param_calculate.sql"
    )
    .fetch_all(pool)
    .await?;

    let GasAnalysisCalculatedParamInterimNestedRows {
        id,
        gas_gravity,
        higher_heating_value,
        carbon_content,
        created_by_id,
        created_at,
        updated_by_id,
        updated_at,
    } = GasAnalysisCalculatedParamInterimUnnestedRows {
        user_id,
        gas_analysis_calculated_params_interim,
    }
    .into();

    let rows_inserted = query_file!(
        "src/graphql/sql/statements/gas_analysis_calculated_param_insert.sql",
        &id,
        &gas_gravity,
        &higher_heating_value,
        &carbon_content,
        &created_by_id,
        &created_at,
        &updated_by_id,
        &updated_at
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_inserted)
}
