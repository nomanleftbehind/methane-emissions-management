use crate::graphql::models::routine::pneumatic_device::DeviceManufacturer;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct DeviceManufacturerLoader {
    pool: Data<PgPool>,
}

impl DeviceManufacturerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for DeviceManufacturerLoader {
    type Value = DeviceManufacturer;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let device_manufacturers = query_as!(
            DeviceManufacturer,
            "SELECT * FROM device_manufacturer WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|device_manufacturer| (device_manufacturer.id, device_manufacturer))
        .collect();

        Ok(device_manufacturers)
    }
}

pub struct CreatedDeviceManufacturersLoader {
    pool: Data<PgPool>,
}

impl CreatedDeviceManufacturersLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedDeviceManufacturersLoader {
    type Value = Vec<DeviceManufacturer>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut device_manufacturers = query_as!(
            DeviceManufacturer,
            "SELECT * FROM device_manufacturer WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        device_manufacturers.sort_by_key(|cf| cf.created_by_id);

        let device_manufacturers = device_manufacturers
            .into_iter()
            .group_by(|device_manufacturer| device_manufacturer.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(device_manufacturers)
    }
}

pub struct UpdatedDeviceManufacturersLoader {
    pool: Data<PgPool>,
}

impl UpdatedDeviceManufacturersLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedDeviceManufacturersLoader {
    type Value = Vec<DeviceManufacturer>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut device_manufacturers = query_as!(
            DeviceManufacturer,
            "SELECT * FROM device_manufacturer WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        device_manufacturers.sort_by_key(|device_manufacturer| device_manufacturer.updated_by_id);

        let device_manufacturers = device_manufacturers
            .into_iter()
            .group_by(|device_manufacturer| device_manufacturer.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(device_manufacturers)
    }
}
