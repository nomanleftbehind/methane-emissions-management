use crate::graphql::models::routine::defined_vent_gas::storage_tank::StorageTank;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct StorageTankLoader {
    pool: Data<PgPool>,
}

impl StorageTankLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for StorageTankLoader {
    type Value = StorageTank;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let storage_tanks = query_as!(
            StorageTank,
            "SELECT * FROM storage_tank WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|storage_tank| (storage_tank.id, storage_tank))
        .collect();

        Ok(storage_tanks)
    }
}

pub struct CreatedStorageTanksLoader {
    pool: Data<PgPool>,
}

impl CreatedStorageTanksLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedStorageTanksLoader {
    type Value = Vec<StorageTank>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tanks = query_as!(
            StorageTank,
            "SELECT * FROM storage_tank WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        storage_tanks.sort_by_key(|storage_tank| storage_tank.created_by_id);

        let storage_tanks = storage_tanks
            .into_iter()
            .group_by(|storage_tank| storage_tank.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(storage_tanks)
    }
}

pub struct UpdatedStorageTanksLoader {
    pool: Data<PgPool>,
}

impl UpdatedStorageTanksLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedStorageTanksLoader {
    type Value = Vec<StorageTank>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tanks = query_as!(
            StorageTank,
            "SELECT * FROM storage_tank WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        storage_tanks.sort_by_key(|storage_tank| storage_tank.updated_by_id);

        let storage_tanks = storage_tanks
            .into_iter()
            .group_by(|storage_tank| storage_tank.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(storage_tanks)
    }
}

pub struct SiteStorageTanksLoader {
    pool: Data<PgPool>,
}

impl SiteStorageTanksLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for SiteStorageTanksLoader {
    type Value = Vec<StorageTank>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tanks = query_as!(
            StorageTank,
            "SELECT * FROM storage_tank WHERE site_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        storage_tanks.sort_by_key(|storage_tank| storage_tank.site_id);

        let storage_tanks = storage_tanks
            .into_iter()
            .group_by(|storage_tank| storage_tank.site_id)
            .into_iter()
            .map(|(site_id, group)| (site_id, group.collect()))
            .collect();

        Ok(storage_tanks)
    }
}
