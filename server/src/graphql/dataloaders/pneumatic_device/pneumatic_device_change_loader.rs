use crate::graphql::models::pneumatic_device::PneumaticDeviceChange;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticDeviceChangeLoader {
    pool: Data<PgPool>,
}

impl PneumaticDeviceChangeLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticDeviceChangeLoader {
    type Value = PneumaticDeviceChange;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_device_changes = query_as!(
            PneumaticDeviceChange,
            "SELECT * FROM pneumatic_device_change WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_device_change| (pneumatic_device_change.id, pneumatic_device_change))
        .collect();

        Ok(pneumatic_device_changes)
    }
}

pub struct CreatedPneumaticDeviceChangesLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticDeviceChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedPneumaticDeviceChangesLoader {
    type Value = Vec<PneumaticDeviceChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_device_changes = query_as!(
            PneumaticDeviceChange,
            "SELECT * FROM pneumatic_device_change WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_device_changes
            .sort_by_key(|pneumatic_device_change| pneumatic_device_change.created_by_id);

        let pneumatic_device_changes = pneumatic_device_changes
            .into_iter()
            .group_by(|pneumatic_device_change| pneumatic_device_change.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(pneumatic_device_changes)
    }
}

pub struct UpdatedPneumaticDeviceChangesLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticDeviceChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedPneumaticDeviceChangesLoader {
    type Value = Vec<PneumaticDeviceChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_device_changes = query_as!(
            PneumaticDeviceChange,
            "SELECT * FROM pneumatic_device_change WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_device_changes.sort_by_key(|pneumatic_device| pneumatic_device.updated_by_id);

        let pneumatic_device_changes = pneumatic_device_changes
            .into_iter()
            .group_by(|pneumatic_device| pneumatic_device.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(pneumatic_device_changes)
    }
}

pub struct PneumaticDeviceChangesByPneumaticDeviceLoader {
    pool: Data<PgPool>,
}

impl PneumaticDeviceChangesByPneumaticDeviceLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticDeviceChangesByPneumaticDeviceLoader {
    type Value = Vec<PneumaticDeviceChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_device_change = query_as!(
            PneumaticDeviceChange,
            "SELECT * FROM pneumatic_device_change WHERE pneumatic_device_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_device_change
            .sort_by_key(|pneumatic_device_change| pneumatic_device_change.pneumatic_device_id);

        let pneumatic_device_changes = pneumatic_device_change
            .into_iter()
            .group_by(|pneumatic_device_change| pneumatic_device_change.pneumatic_device_id)
            .into_iter()
            .map(|(pneumatic_device_id, group)| (pneumatic_device_id, group.collect()))
            .collect();

        Ok(pneumatic_device_changes)
    }
}
