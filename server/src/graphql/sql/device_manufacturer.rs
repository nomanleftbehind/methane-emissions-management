use crate::graphql::models::IdSelection;
use sqlx::{query_as, PgPool};

pub async fn get_device_manufacturers(pool: &PgPool) -> Result<Vec<IdSelection>, sqlx::Error> {
    let get_device_manufacturers = query_as!(
        IdSelection,
        "SELECT id, manufacturer as name FROM device_manufacturer"
    )
    .fetch_all(pool)
    .await;

    get_device_manufacturers
}
