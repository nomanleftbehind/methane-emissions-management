use crate::graphql::models::{
    CompressorMonthVent,
    CompressorMonthVentBy::{self, CompressorId, Month},
    CompressorMonthVentCalculated, CompressorMonthVentNestedRows, CompressorMonthVentUnnestedRows,
};
use chrono::NaiveDate;
use sqlx::{query_as, query_file, query_file_as, Error, PgPool};
use uuid::Uuid;

pub async fn select_compressor_month_vents(
    pool: &PgPool,
    by: CompressorMonthVentBy,
) -> Result<Vec<CompressorMonthVent>, Error> {
    match by {
        CompressorId(id) => {
            query_as!(
                CompressorMonthVent,
                "SELECT * FROM compressor_month_vent WHERE compressor_id = $1",
                id
            )
            .fetch_all(pool)
            .await
        }
        Month(month) => {
            query_as!(
                CompressorMonthVent,
                "SELECT * FROM compressor_month_vent WHERE month = $1",
                month
            )
            .fetch_all(pool)
            .await
        }
    }
}

pub async fn insert_compressor_month_vents(
    pool: &PgPool,
    user_id: Uuid,
    month: NaiveDate,
    c1: &f64,
    co2: &f64,
) -> Result<u64, sqlx::Error> {
    let compressor_month_vents_calculated = query_file_as!(
        CompressorMonthVentCalculated,
        "src/graphql/sql/statements/compressor_month_vent_calculate.sql",
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
