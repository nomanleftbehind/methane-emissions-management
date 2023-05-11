use crate::graphql::models::routine::defined_vent_gas::storage_tank::StorageTankChange;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct StorageTankChangeLoader {
    pool: Data<PgPool>,
}

impl StorageTankChangeLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for StorageTankChangeLoader {
    type Value = StorageTankChange;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let storage_tank_changes = query_as!(
            StorageTankChange,
            r#"
            SELECT
            id, storage_tank_id, date, ia, api_density, temperature, pressure, calculation_method as "calculation_method: _", created_by_id, created_at, updated_by_id, updated_at
            FROM storage_tank_change
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|storage_tank_change| (storage_tank_change.id, storage_tank_change))
        .collect();

        Ok(storage_tank_changes)
    }
}

pub struct CreatedStorageTankChangesLoader {
    pool: Data<PgPool>,
}

impl CreatedStorageTankChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedStorageTankChangesLoader {
    type Value = Vec<StorageTankChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_changes = query_as!(
            StorageTankChange,
            r#"
            SELECT
            id, storage_tank_id, date, ia, api_density, temperature, pressure, calculation_method as "calculation_method: _", created_by_id, created_at, updated_by_id, updated_at
            FROM storage_tank_change
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_changes.sort_by_key(|storage_tank_change| storage_tank_change.created_by_id);

        let storage_tank_changes = storage_tank_changes
            .into_iter()
            .group_by(|storage_tank_change| storage_tank_change.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(storage_tank_changes)
    }
}

pub struct UpdatedStorageTankChangesLoader {
    pool: Data<PgPool>,
}

impl UpdatedStorageTankChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedStorageTankChangesLoader {
    type Value = Vec<StorageTankChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_changes = query_as!(
            StorageTankChange,
            r#"
            SELECT
            id, storage_tank_id, date, ia, api_density, temperature, pressure, calculation_method as "calculation_method: _", created_by_id, created_at, updated_by_id, updated_at
            FROM storage_tank_change
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_changes.sort_by_key(|storage_tank_change| storage_tank_change.updated_by_id);

        let storage_tank_changes = storage_tank_changes
            .into_iter()
            .group_by(|storage_tank_change| storage_tank_change.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(storage_tank_changes)
    }
}

pub struct StorageTankChangesByStorageTankLoader {
    pool: Data<PgPool>,
}

impl StorageTankChangesByStorageTankLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for StorageTankChangesByStorageTankLoader {
    type Value = Vec<StorageTankChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_changes = query_as!(
            StorageTankChange,
            r#"
            SELECT
            id, storage_tank_id, date, ia, api_density, temperature, pressure, calculation_method as "calculation_method: _", created_by_id, created_at, updated_by_id, updated_at
            FROM storage_tank_change
            WHERE storage_tank_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_changes.sort_by_key(|storage_tank_change| storage_tank_change.storage_tank_id);

        let storage_tank_changes = storage_tank_changes
            .into_iter()
            .group_by(|storage_tank_change| storage_tank_change.storage_tank_id)
            .into_iter()
            .map(|(storage_tank_id, group)| (storage_tank_id, group.collect()))
            .collect();

        Ok(storage_tank_changes)
    }
}
