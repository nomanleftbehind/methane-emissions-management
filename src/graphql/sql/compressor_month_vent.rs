use crate::{
    configuration::DefaultMoleFractions,
    graphql::models::{
        CompressorMonthVentCalculated, CompressorMonthVentNestedRows,
        CompressorMonthVentUnnestedRows,
    },
};
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_compressor_month_vents(
    pool: &PgPool,
    user_id: Uuid,
    month: NaiveDate,
    DefaultMoleFractions { c1, co2 }: &DefaultMoleFractions,
) -> Result<u64, sqlx::Error> {
    let compressor_month_vents_calculated = sqlx::query_file_as!(
        CompressorMonthVentCalculated,
        "src/graphql/sql/statements/compressor_month_vent_calculated.sql",
        month,
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

    let rows_inserted = sqlx::query_file!(
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
