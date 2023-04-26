use crate::graphql::models::InsertControllerInput;
use sqlx::{query, Error, PgPool};
use uuid::Uuid;

pub async fn insert_controller(
    pool: &PgPool,
    user_id: Uuid,
    InsertControllerInput {
        fdc_rec_id,
        manufacturer_id,
        model,
        serial_number,
        application_id,
        facility_id,
    }: InsertControllerInput,
) -> Result<u64, Error> {
    let rows_inserted = query!(
        "INSERT INTO pneumatic_device (id, site_id, type, manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);",
        Uuid::new_v4(),
        fdc_rec_id,
        manufacturer_id,
        model,
        serial_number,
        application_id,
        facility_id,
        user_id,
        chrono::Utc::now().naive_utc(),
        user_id,
        chrono::Utc::now().naive_utc())
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_inserted)
}
