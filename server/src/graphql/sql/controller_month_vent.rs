use crate::graphql::models::{
    ControllerMonthVent,
    ControllerMonthVentBy::{self, ControllerId, Month},
    ControllerMonthVentCalculated, ControllerMonthVentNestedRows, ControllerMonthVentUnnestedRows,
};
use chrono::NaiveDate;
use sqlx::{query_as, query_file, query_file_as, Error, PgPool};
use uuid::Uuid;

pub async fn select_controller_month_vents(
    pool: &PgPool,
    by: ControllerMonthVentBy,
) -> Result<Vec<ControllerMonthVent>, Error> {
    match by {
        ControllerId(id) => {
            query_as!(
                ControllerMonthVent,
                "SELECT * FROM controller_month_vent WHERE controller_id = $1",
                id
            )
            .fetch_all(pool)
            .await
        }
        Month(month) => {
            query_as!(
                ControllerMonthVent,
                "SELECT * FROM controller_month_vent WHERE month = $1",
                month
            )
            .fetch_all(pool)
            .await
        }
    }
}

pub async fn insert_controller_month_vents(
    pool: &PgPool,
    user_id: Uuid,
    months: &[NaiveDate],
    c1: &f64,
    co2: &f64,
) -> Result<u64, Error> {
    let controller_month_vents_calculated = query_file_as!(
        ControllerMonthVentCalculated,
        "src/graphql/sql/statements/controller_month_vent_calculate.sql",
        months,
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