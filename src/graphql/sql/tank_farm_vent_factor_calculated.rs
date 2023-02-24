use crate::graphql::models::{
    TankFarmVentFactorCalculatedInterim, TankFarmVentFactorCalculatedInterimNestedRows,
    TankFarmVentFactorCalculatedInterimUnnestedRows,
};
use sqlx::{query_file, query_file_as, Error, PgPool};
use uuid::Uuid;

pub async fn insert_tank_farm_vent_factors_calculated(
    pool: &PgPool,
    user_id: Uuid,
    gas_gravity: &f64,
) -> Result<u64, Error> {
    let tank_farm_vent_factors_calculated_interim = query_file_as!(
        TankFarmVentFactorCalculatedInterim,
        "src/graphql/sql/statements/tank_farm_vent_factor_calculate.sql",
        gas_gravity
    )
    .fetch_all(pool)
    .await?;

    let TankFarmVentFactorCalculatedInterimNestedRows {
        id,
        tank_farm_id,
        date,
        vent_factor,
        created_by_id,
        created_at,
        updated_by_id,
        updated_at,
    } = TankFarmVentFactorCalculatedInterimUnnestedRows {
        user_id,
        tank_farm_vent_factors_calculated_interim,
    }
    .into();

    let rows_inserted = query_file!(
        "src/graphql/sql/statements/tank_farm_vent_factor_insert.sql",
        &id,
        &tank_farm_id,
        &date,
        &vent_factor,
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
