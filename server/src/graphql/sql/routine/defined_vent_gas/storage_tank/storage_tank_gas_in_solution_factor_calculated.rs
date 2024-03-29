use super::super::super::super::gas_analysis::insert_gas_analysis_calculated_param;
use crate::graphql::models::{
    input::MonthRangeInput,
    routine::defined_vent_gas::storage_tank::{
        StorageTankGasInSolutionFactorCalculatedInterim,
        StorageTankGasInSolutionFactorCalculatedInterimNestedRows,
        StorageTankGasInSolutionFactorCalculatedInterimUnnestedRows,
    },
};
use sqlx::{query_file, query_file_as, Error, PgPool};
use uuid::Uuid;

pub async fn insert_storage_tank_gas_in_solution_factor_calculated(
    pool: &PgPool,
    user_id: &Uuid,
    month_range: &MonthRangeInput,
    gas_gravity: &f64,
) -> Result<u64, Error> {
    // Gas in solution factor calculation requires precalculation of gas gravity which is done here.
    insert_gas_analysis_calculated_param(pool, user_id, month_range).await?;

    let MonthRangeInput {
        from_month,
        to_month,
    } = month_range;

    let storage_tank_gas_in_solution_factors_calculated_interim = query_file_as!(
        StorageTankGasInSolutionFactorCalculatedInterim,
        "src/graphql/sql/statements/storage_tank_gas_in_solution_factor_calculate.sql",
        from_month,
        to_month,
        gas_gravity
    )
    .fetch_all(pool)
    .await?;

    let StorageTankGasInSolutionFactorCalculatedInterimNestedRows {
        id,
        storage_tank_id,
        date,
        gis_factor,
        created_at,
    } = StorageTankGasInSolutionFactorCalculatedInterimUnnestedRows(
        storage_tank_gas_in_solution_factors_calculated_interim,
    )
    .into();

    let rows_inserted = query_file!(
        "src/graphql/sql/statements/storage_tank_gas_in_solution_factor_insert.sql",
        &id,
        &storage_tank_id,
        &date,
        &gis_factor,
        &created_at,
        user_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_inserted)
}
