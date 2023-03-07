use crate::graphql::models::{
    TankFarmMonthVent,
    TankFarmMonthVentBy::{self, Month, TankFarmId},
    TankFarmMonthVentInterim, TankFarmMonthVentInterimNestedRows,
    TankFarmMonthVentInterimUnnestedRows,
};
use chrono::NaiveDate;
use sqlx::{query_as, query_file, query_file_as, Error, PgPool};
use uuid::Uuid;

pub async fn select_tank_farm_month_vents(
    pool: &PgPool,
    by: TankFarmMonthVentBy,
) -> Result<Vec<TankFarmMonthVent>, Error> {
    match by {
        TankFarmId(id) => {
            query_as!(
                TankFarmMonthVent,
                "SELECT * FROM tank_farm_month_vent WHERE tank_farm_id = $1",
                id
            )
            .fetch_all(pool)
            .await
        }
        Month(month) => {
            query_as!(
                TankFarmMonthVent,
                "SELECT * FROM tank_farm_month_vent WHERE month = $1",
                month
            )
            .fetch_all(pool)
            .await
        }
    }
}

pub async fn insert_tank_farm_month_vents(
    pool: &PgPool,
    user_id: Uuid,
    months: &[NaiveDate],
    c1: &f64,
    co2: &f64,
) -> Result<u64, Error> {
    let tank_farm_month_vents_interim = query_file_as!(
        TankFarmMonthVentInterim,
        "src/graphql/sql/statements/tank_farm_month_vent_calculate.sql",
        months,
        c1,
        co2
    )
    .fetch_all(pool)
    .await?;

    let TankFarmMonthVentInterimNestedRows {
        id,
        tank_farm_id,
        month,
        gas_volume,
        c1_volume,
        co2_volume,
        created_by_id,
        created_at,
        updated_by_id,
        updated_at,
    } = TankFarmMonthVentInterimUnnestedRows {
        user_id,
        tank_farm_month_vents_interim,
    }
    .into();

    let rows_inserted = query_file!(
        "src/graphql/sql/statements/tank_farm_month_vent_insert.sql",
        &id,
        &tank_farm_id,
        &month,
        &gas_volume,
        &c1_volume,
        &co2_volume,
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
