use super::super::routine::defined_vent_gas::storage_tank::insert_storage_tank_gas_in_solution_factor_calculated;
use crate::graphql::models::{
    input::{MonthMethaneEmissionBySourceIdInput, MonthRangeInput},
    month_methane_emission::{
        MonthMethaneEmission, MonthMethaneEmissionCalculated, MonthMethaneEmissionNestedRows,
        MonthMethaneEmissionUnnestedRows,
    },
};
use common::{MethaneEmissionCategory, MethaneEmissionSource, MethaneEmissionSourceTable};
use sqlx::{query_as, query_file, query_file_as, Error, PgPool};
use uuid::Uuid;

pub async fn get_month_methane_emissions(
    pool: &PgPool,
    by: MonthMethaneEmissionBySourceIdInput,
) -> Result<Vec<MonthMethaneEmission>, Error> {
    query_as!(MonthMethaneEmission,
        r#"
        SELECT
        id, facility_id, site_id, source_table as "source_table: _", source_table_id, category as "category: _", source as "source: _", month, gas_volume, c1_volume, co2_volume, created_by_id, created_at, updated_by_id, updated_at
        FROM month_methane_emission
        WHERE source_table_id = $1
        "#,
        by.source_id
    )
    .fetch_all(pool)
    .await
}

pub async fn insert_month_methane_emissions(
    pool: &PgPool,
    user_id: Uuid,
    month_range: &MonthRangeInput,
    c1: &f64,
    co2: &f64,
    gas_gravity: &f64,
) -> Result<u64, Error> {
    let MonthRangeInput {
        from_month,
        to_month,
    } = month_range;

    let mut pneumatic_instrument_month_methane_emissions_calculated = query_file_as!(
        MonthMethaneEmissionCalculated,
        "src/graphql/sql/statements/pneumatic_instrument_month_methane_emission_calculate.sql",
        from_month,
        to_month,
        c1,
        co2
    )
    .fetch_all(pool)
    .await?;

    let mut level_controller_month_methane_emissions_calculated = query_file_as!(
        MonthMethaneEmissionCalculated,
        "src/graphql/sql/statements/level_controller_month_methane_emission_calculate.sql",
        from_month,
        to_month,
        c1,
        co2
    )
    .fetch_all(pool)
    .await?;

    let mut pneumatic_pump_month_methane_emissions_calculated = query_file_as!(
        MonthMethaneEmissionCalculated,
        "src/graphql/sql/statements/pneumatic_pump_month_methane_emission_calculate.sql",
        from_month,
        to_month,
        c1,
        co2
    )
    .fetch_all(pool)
    .await?;

    let mut compressor_seal_month_methane_emissions_calculated = query_file_as!(
        MonthMethaneEmissionCalculated,
        "src/graphql/sql/statements/compressor_seal_month_methane_emission_calculate.sql",
        from_month,
        to_month,
        c1,
        co2
    )
    .fetch_all(pool)
    .await?;

    let mut compressor_blowdown_month_methane_emissions_calculated = query_file_as!(
        MonthMethaneEmissionCalculated,
        "src/graphql/sql/statements/compressor_blowdown_month_methane_emission_calculate.sql",
        from_month,
        to_month,
        c1,
        co2
    )
    .fetch_all(pool)
    .await?;

    // Storage tank methane emissions calculation requires precalculation of GIS factors which is done here.
    insert_storage_tank_gas_in_solution_factor_calculated(pool, user_id, month_range, gas_gravity)
        .await?;

    let mut storage_tank_month_methane_emissions_calculated = query_file_as!(
        MonthMethaneEmissionCalculated,
        "src/graphql/sql/statements/storage_tank_month_methane_emission_calculate.sql",
        from_month,
        to_month,
        c1,
        co2,
        0.9475
    )
    .fetch_all(pool)
    .await?;

    pneumatic_instrument_month_methane_emissions_calculated
        .append(&mut level_controller_month_methane_emissions_calculated);

    pneumatic_instrument_month_methane_emissions_calculated
        .append(&mut pneumatic_pump_month_methane_emissions_calculated);

    pneumatic_instrument_month_methane_emissions_calculated
        .append(&mut compressor_seal_month_methane_emissions_calculated);

    pneumatic_instrument_month_methane_emissions_calculated
        .append(&mut storage_tank_month_methane_emissions_calculated);

    pneumatic_instrument_month_methane_emissions_calculated
        .append(&mut compressor_blowdown_month_methane_emissions_calculated);

    let MonthMethaneEmissionNestedRows {
        id,
        facility_id,
        site_id,
        source_table,
        source_table_id,
        category,
        source,
        month,
        gas_volume,
        c1_volume,
        co2_volume,
        created_at,
    } = MonthMethaneEmissionUnnestedRows(pneumatic_instrument_month_methane_emissions_calculated)
        .into();

    let rows_inserted = query_file!(
        "src/graphql/sql/statements/month_methane_emission_insert.sql",
        &id,
        &facility_id,
        &site_id,
        &source_table as &[MethaneEmissionSourceTable],
        &source_table_id,
        &category as &[MethaneEmissionCategory],
        &source as &[MethaneEmissionSource],
        &month,
        &gas_volume,
        &c1_volume,
        &co2_volume,
        &created_at,
        user_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_inserted)
}
