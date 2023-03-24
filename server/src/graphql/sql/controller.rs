use crate::graphql::models::{Controller, EmittersByInput};
use sqlx::{query, query_as, Error, PgPool};
use uuid::Uuid;

pub async fn query_controllers(
    pool: &PgPool,
    EmittersByInput { facility_id }: EmittersByInput,
) -> Result<Vec<Controller>, Error> {
    query_as!(
        Controller,
        "SELECT * FROM controllers WHERE facility_id = $1",
        facility_id
    )
    .fetch_all(pool)
    .await
}

pub async fn update_controller_serial_number(
    pool: &PgPool,
    id: Uuid,
    value: Option<String>,
    updated_by_id: Uuid,
) -> Result<u64, Error> {
    Ok(query!(
        "UPDATE controllers
        SET serial_number = $2,
            updated_by_id = $3,
            updated_at = $4
        WHERE id = $1",
        id,
        value,
        updated_by_id,
        chrono::Utc::now().naive_utc()
    )
    .execute(pool)
    .await?
    .rows_affected())
}
