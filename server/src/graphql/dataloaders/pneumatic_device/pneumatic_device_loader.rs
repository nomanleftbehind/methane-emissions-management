use crate::graphql::models::pneumatic_device::PneumaticDevice;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticDeviceLoader {
    pool: Data<PgPool>,
}

impl PneumaticDeviceLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticDeviceLoader {
    type Value = PneumaticDevice;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_devices = sqlx::query_as!(
            PneumaticDevice,
            r#"SELECT id, site_id, type as "type: _", manufacturer_id, model, serial_number, created_by_id, created_at, updated_by_id, updated_at FROM pneumatic_device WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_device| (pneumatic_device.id, pneumatic_device))
        .collect();

        Ok(pneumatic_devices)
    }
}

pub struct CreatedPneumaticDevicesLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticDevicesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedPneumaticDevicesLoader {
    type Value = Vec<PneumaticDevice>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_devices = sqlx::query_as!(
            PneumaticDevice,
            r#"SELECT id, site_id, type as "type: _", manufacturer_id, model, serial_number, created_by_id, created_at, updated_by_id, updated_at FROM pneumatic_device WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        pneumatic_devices.sort_by_key(|pneumatic_device| pneumatic_device.created_by_id);

        let created_pneumatic_devices = pneumatic_devices
            .into_iter()
            .group_by(|pneumatic_device| pneumatic_device.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_pneumatic_devices)
    }
}

pub struct UpdatedPneumaticDevicesLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticDevicesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedPneumaticDevicesLoader {
    type Value = Vec<PneumaticDevice>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_devices = sqlx::query_as!(
            PneumaticDevice,
            r#"SELECT id, site_id, type as "type: _", manufacturer_id, model, serial_number, created_by_id, created_at, updated_by_id, updated_at FROM pneumatic_device WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        pneumatic_devices.sort_by_key(|pneumatic_device| pneumatic_device.updated_by_id);

        let updated_pneumatic_devices = pneumatic_devices
            .into_iter()
            .group_by(|pneumatic_device| pneumatic_device.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_pneumatic_devices)
    }
}

pub struct SitePneumaticDevicesLoader {
    pool: Data<PgPool>,
}

impl SitePneumaticDevicesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for SitePneumaticDevicesLoader {
    type Value = Vec<PneumaticDevice>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_devices = sqlx::query_as!(
            PneumaticDevice,
            r#"SELECT id, site_id, type as "type: _", manufacturer_id, model, serial_number, created_by_id, created_at, updated_by_id, updated_at FROM pneumatic_device WHERE site_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        pneumatic_devices.sort_by_key(|pneumatic_device| pneumatic_device.site_id);

        let pneumatic_devices = pneumatic_devices
            .into_iter()
            .group_by(|pneumatic_device| pneumatic_device.site_id)
            .into_iter()
            .map(|(facility_id, group)| (facility_id, group.collect()))
            .collect();

        Ok(pneumatic_devices)
    }
}

pub struct PneumaticDevicesByManufacturerLoader {
    pool: Data<PgPool>,
}

impl PneumaticDevicesByManufacturerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticDevicesByManufacturerLoader {
    type Value = Vec<PneumaticDevice>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_devices = sqlx::query_as!(
            PneumaticDevice,
            r#"SELECT id, site_id, type as "type: _", manufacturer_id, model, serial_number, created_by_id, created_at, updated_by_id, updated_at FROM pneumatic_device WHERE manufacturer_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        pneumatic_devices.sort_by_key(|pneumatic_device| pneumatic_device.manufacturer_id);

        let pneumatic_devices = pneumatic_devices
            .into_iter()
            .group_by(|pneumatic_device| pneumatic_device.manufacturer_id)
            .into_iter()
            .map(|(manufacturer_id, group)| (manufacturer_id, group.collect()))
            .collect();

        Ok(pneumatic_devices)
    }
}
