use crate::graphql::models::routine::defined_vent_gas::storage_tank::StorageTankControlDeviceInactivity;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct StorageTankControlDeviceInactivityLoader {
    pool: Data<PgPool>,
}

impl StorageTankControlDeviceInactivityLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for StorageTankControlDeviceInactivityLoader {
    type Value = StorageTankControlDeviceInactivity;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let storage_tank_control_device_inactivities = query_as!(
            StorageTankControlDeviceInactivity,
            r#"
            SELECT
            id, storage_tank_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM storage_tank_control_device_inactivity
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|storage_tank_control_device_inactivity| (storage_tank_control_device_inactivity.id, storage_tank_control_device_inactivity))
        .collect();

        Ok(storage_tank_control_device_inactivities)
    }
}

pub struct CreatedStorageTankControlDeviceInactivitiesLoader {
    pool: Data<PgPool>,
}

impl CreatedStorageTankControlDeviceInactivitiesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedStorageTankControlDeviceInactivitiesLoader {
    type Value = Vec<StorageTankControlDeviceInactivity>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_control_device_inactivities = query_as!(
            StorageTankControlDeviceInactivity,
            r#"
            SELECT
            id, storage_tank_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM storage_tank_control_device_inactivity
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_control_device_inactivities.sort_by_key(
            |storage_tank_control_device_inactivity| {
                storage_tank_control_device_inactivity.created_by_id
            },
        );

        let storage_tank_control_device_inactivities = storage_tank_control_device_inactivities
            .into_iter()
            .group_by(|storage_tank_control_device_inactivity| {
                storage_tank_control_device_inactivity.created_by_id
            })
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(storage_tank_control_device_inactivities)
    }
}

pub struct UpdatedStorageTankControlDeviceInactivitiesLoader {
    pool: Data<PgPool>,
}

impl UpdatedStorageTankControlDeviceInactivitiesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedStorageTankControlDeviceInactivitiesLoader {
    type Value = Vec<StorageTankControlDeviceInactivity>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_control_device_inactivities = query_as!(
            StorageTankControlDeviceInactivity,
            r#"
            SELECT
            id, storage_tank_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM storage_tank_control_device_inactivity
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_control_device_inactivities.sort_by_key(
            |storage_tank_control_device_inactivity| {
                storage_tank_control_device_inactivity.updated_by_id
            },
        );

        let storage_tank_control_device_inactivities = storage_tank_control_device_inactivities
            .into_iter()
            .group_by(|storage_tank_control_device_inactivity| {
                storage_tank_control_device_inactivity.updated_by_id
            })
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(storage_tank_control_device_inactivities)
    }
}

pub struct StorageTankControlDeviceInactivitiesByStorageTankControlledCharacterizationLoader {
    pool: Data<PgPool>,
}

impl StorageTankControlDeviceInactivitiesByStorageTankControlledCharacterizationLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid>
    for StorageTankControlDeviceInactivitiesByStorageTankControlledCharacterizationLoader
{
    type Value = Vec<StorageTankControlDeviceInactivity>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_control_device_inactivities = query_as!(
            StorageTankControlDeviceInactivity,
            r#"
            SELECT
            id, storage_tank_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM storage_tank_control_device_inactivity
            WHERE storage_tank_controlled_characterization_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_control_device_inactivities.sort_by_key(
            |storage_tank_control_device_inactivity| {
                storage_tank_control_device_inactivity.storage_tank_controlled_characterization_id
            },
        );

        let storage_tank_control_device_inactivities = storage_tank_control_device_inactivities
            .into_iter()
            .group_by(|storage_tank_control_device_inactivity| {
                storage_tank_control_device_inactivity.storage_tank_controlled_characterization_id
            })
            .into_iter()
            .map(|(storage_tank_controlled_characterization_id, group)| {
                (storage_tank_controlled_characterization_id, group.collect())
            })
            .collect();

        Ok(storage_tank_control_device_inactivities)
    }
}
