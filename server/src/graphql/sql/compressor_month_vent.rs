use crate::graphql::models::compressor::{
    CompressorMonthVent,
    CompressorMonthVentBy::{self, CompressorId, Month},
    CompressorMonthVentCalculated, CompressorMonthVentNestedRows, CompressorMonthVentUnnestedRows,
};
use chrono::NaiveDate;
use sqlx::{query_as, query_file, query_file_as, Error, PgPool};
use uuid::Uuid;

pub async fn insert_compressor_month_vents(
    pool: &PgPool,
    user_id: Uuid,
    months: &[NaiveDate],
    c1: &f64,
    co2: &f64,
) -> Result<u64, sqlx::Error> {
    let compressor_month_vents_calculated = query_file_as!(
        CompressorMonthVentCalculated,
        "src/graphql/sql/statements/compressor_month_vent_calculate.sql",
        months,
        c1,
        co2
    )
    .fetch_all(pool)
    .await?;

    let CompressorMonthVentNestedRows {
        id,
        month,
        gas_volume,
        c1_volume,
        co2_volume,
        compressor_id,
        created_by_id,
        created_at,
        updated_by_id,
        updated_at,
    } = CompressorMonthVentUnnestedRows {
        user_id,
        compressor_month_vents_calculated,
    }
    .into();

    let rows_inserted = query_file!(
        "src/graphql/sql/statements/compressor_month_vent_insert.sql",
        &id,
        &month,
        &gas_volume,
        &c1_volume,
        &co2_volume,
        &compressor_id,
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
