use crate::graphql::models::routine::defined_vent_gas::storage_tank::StorageTankControlledCharacterization;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct StorageTankControlledCharacterizationLoader {
    pool: Data<PgPool>,
}

impl StorageTankControlledCharacterizationLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for StorageTankControlledCharacterizationLoader {
    type Value = StorageTankControlledCharacterization;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let storage_tank_controlled_characterizations = query_as!(
            StorageTankControlledCharacterization,
            r#"
            SELECT
            id, storage_tank_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM storage_tank_controlled_characterization
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|storage_tank_controlled_characterization| (storage_tank_controlled_characterization.id, storage_tank_controlled_characterization))
        .collect();

        Ok(storage_tank_controlled_characterizations)
    }
}

pub struct CreatedStorageTankControlledCharacterizationsLoader {
    pool: Data<PgPool>,
}

impl CreatedStorageTankControlledCharacterizationsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedStorageTankControlledCharacterizationsLoader {
    type Value = Vec<StorageTankControlledCharacterization>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_controlled_characterizations = query_as!(
            StorageTankControlledCharacterization,
            r#"
            SELECT
            id, storage_tank_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM storage_tank_controlled_characterization
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_controlled_characterizations.sort_by_key(
            |storage_tank_controlled_characterization| {
                storage_tank_controlled_characterization.created_by_id
            },
        );

        let storage_tank_controlled_characterizations = storage_tank_controlled_characterizations
            .into_iter()
            .group_by(|storage_tank_controlled_characterization| {
                storage_tank_controlled_characterization.created_by_id
            })
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(storage_tank_controlled_characterizations)
    }
}

pub struct UpdatedStorageTankControlledCharacterizationsLoader {
    pool: Data<PgPool>,
}

impl UpdatedStorageTankControlledCharacterizationsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedStorageTankControlledCharacterizationsLoader {
    type Value = Vec<StorageTankControlledCharacterization>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_controlled_characterizations = query_as!(
            StorageTankControlledCharacterization,
            r#"
            SELECT
            id, storage_tank_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM storage_tank_controlled_characterization
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_controlled_characterizations.sort_by_key(
            |storage_tank_controlled_characterization| {
                storage_tank_controlled_characterization.updated_by_id
            },
        );

        let storage_tank_controlled_characterizations = storage_tank_controlled_characterizations
            .into_iter()
            .group_by(|storage_tank_controlled_characterization| {
                storage_tank_controlled_characterization.updated_by_id
            })
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(storage_tank_controlled_characterizations)
    }
}

pub struct StorageTankControlledCharacterizationsByStorageTankLoader {
    pool: Data<PgPool>,
}

impl StorageTankControlledCharacterizationsByStorageTankLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for StorageTankControlledCharacterizationsByStorageTankLoader {
    type Value = Vec<StorageTankControlledCharacterization>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_controlled_characterizations = query_as!(
            StorageTankControlledCharacterization,
            r#"
            SELECT
            id, storage_tank_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM storage_tank_controlled_characterization
            WHERE storage_tank_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_controlled_characterizations.sort_by_key(
            |storage_tank_controlled_characterization| {
                storage_tank_controlled_characterization.storage_tank_id
            },
        );

        let storage_tank_controlled_characterizations = storage_tank_controlled_characterizations
            .into_iter()
            .group_by(|storage_tank_controlled_characterization| {
                storage_tank_controlled_characterization.storage_tank_id
            })
            .into_iter()
            .map(|(storage_tank_id, group)| (storage_tank_id, group.collect()))
            .collect();

        Ok(storage_tank_controlled_characterizations)
    }
}
