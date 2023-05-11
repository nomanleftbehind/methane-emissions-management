use crate::graphql::models::nonroutine::compressor_blowdown::CompressorBlowdown;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct CompressorBlowdownLoader {
    pool: Data<PgPool>,
}

impl CompressorBlowdownLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorBlowdownLoader {
    type Value = CompressorBlowdown;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let compressor_blowdowns = query_as!(
            CompressorBlowdown,
            "SELECT * FROM compressor_blowdown WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|compressor_blowdown| (compressor_blowdown.id, compressor_blowdown))
        .collect();

        Ok(compressor_blowdowns)
    }
}

pub struct CreatedCompressorBlowdownsLoader {
    pool: Data<PgPool>,
}

impl CreatedCompressorBlowdownsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedCompressorBlowdownsLoader {
    type Value = Vec<CompressorBlowdown>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_blowdowns = query_as!(
            CompressorBlowdown,
            "SELECT * FROM compressor_blowdown WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_blowdowns.sort_by_key(|compressor_blowdown| compressor_blowdown.created_by_id);

        let created_compressor_blowdowns = compressor_blowdowns
            .into_iter()
            .group_by(|compressor_blowdown| compressor_blowdown.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_compressor_blowdowns)
    }
}

pub struct UpdatedCompressorBlowdownsLoader {
    pool: Data<PgPool>,
}

impl UpdatedCompressorBlowdownsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedCompressorBlowdownsLoader {
    type Value = Vec<CompressorBlowdown>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_blowdowns = query_as!(
            CompressorBlowdown,
            "SELECT * FROM compressor_blowdown WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_blowdowns.sort_by_key(|compressor_blowdown| compressor_blowdown.updated_by_id);

        let updated_compressor_blowdowns = compressor_blowdowns
            .into_iter()
            .group_by(|compressor_blowdown| compressor_blowdown.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_compressor_blowdowns)
    }
}

pub struct CompressorBlowdownsByCompressorLoader {
    pool: Data<PgPool>,
}

impl CompressorBlowdownsByCompressorLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorBlowdownsByCompressorLoader {
    type Value = Vec<CompressorBlowdown>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_blowdowns = query_as!(
            CompressorBlowdown,
            "SELECT * FROM compressor_blowdown WHERE compressor_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_blowdowns.sort_by_key(|compressor_blowdown| compressor_blowdown.compressor_id);

        let compressor_blowdowns_by_compressor = compressor_blowdowns
            .into_iter()
            .group_by(|compressor_blowdown| compressor_blowdown.compressor_id)
            .into_iter()
            .map(|(compressor_id, group)| (compressor_id, group.collect()))
            .collect();

        Ok(compressor_blowdowns_by_compressor)
    }
}
