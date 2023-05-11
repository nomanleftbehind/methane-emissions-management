use crate::graphql::models::input::InsertPneumaticDeviceInput;
use common::NonLevelControllerType;
use sqlx::{query, Error, PgPool};
use uuid::Uuid;

pub async fn insert_non_level_controller(
    pool: &PgPool,
    user_id: Uuid,
    InsertPneumaticDeviceInput {
        site_id,
        manufacturer_id,
        model,
        serial_number,
        r#type,
        start_date,
        end_date,
    }: InsertPneumaticDeviceInput,
) -> Result<u64, Error> {
    let rows_inserted = query!(
        "INSERT INTO non_level_controller (id, site_id, type, manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);",
        Uuid::new_v4(),
        site_id,
        r#type as NonLevelControllerType,
        manufacturer_id,
        model,
        serial_number,
        start_date,
        end_date,
        user_id,
        chrono::Utc::now().naive_utc(),
        user_id,
        chrono::Utc::now().naive_utc())
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_inserted)
}
