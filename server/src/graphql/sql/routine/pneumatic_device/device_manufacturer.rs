use crate::graphql::models::routine::pneumatic_device::DeviceManufacturer;
use sqlx::{query_as, PgPool};

pub async fn get_device_manufacturers(
    pool: &PgPool,
) -> Result<Vec<DeviceManufacturer>, sqlx::Error> {
    let get_device_manufacturers =
        query_as!(DeviceManufacturer, "SELECT * FROM device_manufacturer")
            .fetch_all(pool)
            .await;

    get_device_manufacturers
}
