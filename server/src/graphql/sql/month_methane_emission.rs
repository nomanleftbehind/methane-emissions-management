use crate::graphql::models::{
    MonthMethaneEmission, MonthMethaneEmissionBySourceIdInput, MonthMethaneEmissionCalculated,
    MonthMethaneEmissionNestedRows, MonthMethaneEmissionUnnestedRows,
};
use chrono::NaiveDate;
use sqlx::{query_as, query_file, query_file_as, Error, PgPool};
use uuid::Uuid;

pub async fn select_month_methane_emissions(
    pool: &PgPool,
    by: MonthMethaneEmissionBySourceIdInput,
) -> Result<Vec<MonthMethaneEmission>, Error> {
    query_as!(MonthMethaneEmission,
        r#"
        SELECT
        id, facility_id, site_id, source as "source: _", source_id, category as "category: _", month, gas_volume, c1_volume, co2_volume, created_by_id, created_at, updated_by_id, updated_at
        FROM month_methane_emission
        WHERE source_id = $1
        "#,
        by.source_id
    )
    .fetch_all(pool)
    .await
}

pub async fn insert_controller_month_vents(
    pool: &PgPool,
    user_id: Uuid,
    months: &[NaiveDate],
    c1: &f64,
    co2: &f64,
) -> Result<u64, Error> {
    let month_methane_emissions_calculated = query_file_as!(
        MonthMethaneEmissionCalculated,
        "src/graphql/sql/statements/controller_month_vent_calculate.sql",
        months,
        c1,
        co2
    )
    .fetch_all(pool)
    .await?;

    let MonthMethaneEmissionNestedRows {
        id,
        facility_id,
        site_id,
        source,
        source_id,
        category,
        month,
        gas_volume,
        c1_volume,
        co2_volume,
        // created_by_id,
        created_at,
        // updated_by_id,
        updated_at,
    } = MonthMethaneEmissionUnnestedRows {
        user_id,
        month_methane_emissions_calculated,
    }
    .into();

    let rows_inserted = query_file!(
        "src/graphql/sql/statements/controller_month_vent_insert.sql",
        &id,
        &month,
        &gas_volume,
        &c1_volume,
        &co2_volume,
        &controller_id,
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
