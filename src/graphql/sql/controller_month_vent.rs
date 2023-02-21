use crate::{
    configuration::DefaultMoleFractions,
    graphql::models::{
        ControllerMonthVentCalculated, ControllerMonthVentNestedRows,
        ControllerMonthVentUnnestedRows,
    },
};
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_controller_month_vents(
    pool: &PgPool,
    user_id: Uuid,
    month: NaiveDate,
    DefaultMoleFractions { c1, co2 }: &DefaultMoleFractions,
) -> Result<u64, sqlx::Error> {
    let controller_month_vents_calculated = sqlx::query_file_as!(
        ControllerMonthVentCalculated,
        "src/graphql/sql/statements/controller_month_vent_calculated.sql",
        month,
        c1,
        co2
    )
    .fetch_all(pool)
    .await?;

    let ControllerMonthVentNestedRows {
        id,
        month,
        gas_volume,
        c1_volume,
        co2_volume,
        controller_id,
        created_by_id,
        created_at,
        updated_by_id,
        updated_at,
    } = ControllerMonthVentUnnestedRows {
        user_id,
        controller_month_vents_calculated,
    }
    .into();

    let rows_inserted = sqlx::query_file!(
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
