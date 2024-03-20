use crate::graphql::models::nonroutine::compressor_blowdown::CompressorBlowdownOverride;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct CompressorBlowdownOverrideLoader {
    pool: Data<PgPool>,
}

impl CompressorBlowdownOverrideLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CompressorBlowdownOverrideLoader {
    type Value = CompressorBlowdownOverride;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let compressor_blowdown_overrides = query_as!(
            CompressorBlowdownOverride,
            "SELECT * FROM compressor_blowdown_override WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|compressor_blowdown_override| {
            (
                compressor_blowdown_override.id,
                compressor_blowdown_override,
            )
        })
        .collect();

        Ok(compressor_blowdown_overrides)
    }
}

pub struct CreatedCompressorBlowdownOverridesLoader {
    pool: Data<PgPool>,
}

impl CreatedCompressorBlowdownOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedCompressorBlowdownOverridesLoader {
    type Value = Vec<CompressorBlowdownOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_blowdown_overrides = query_as!(
            CompressorBlowdownOverride,
            "SELECT * FROM compressor_blowdown_override WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_blowdown_overrides
            .sort_by_key(|compressor_blowdown_override| compressor_blowdown_override.created_by_id);

        let created_compressor_blowdown_overrides = compressor_blowdown_overrides
            .into_iter()
            .group_by(|compressor_blowdown_override| compressor_blowdown_override.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_compressor_blowdown_overrides)
    }
}

pub struct UpdatedCompressorBlowdownOverridesLoader {
    pool: Data<PgPool>,
}

impl UpdatedCompressorBlowdownOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedCompressorBlowdownOverridesLoader {
    type Value = Vec<CompressorBlowdownOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_blowdown_overrides = query_as!(
            CompressorBlowdownOverride,
            "SELECT * FROM compressor_blowdown_override WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_blowdown_overrides
            .sort_by_key(|compressor_blowdown_override| compressor_blowdown_override.updated_by_id);

        let updated_compressor_blowdown_overrides = compressor_blowdown_overrides
            .into_iter()
            .group_by(|compressor_blowdown_override| compressor_blowdown_override.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_compressor_blowdown_overrides)
    }
}

pub struct CompressorBlowdownOverridesByCompressorLoader {
    pool: Data<PgPool>,
}

impl CompressorBlowdownOverridesByCompressorLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CompressorBlowdownOverridesByCompressorLoader {
    type Value = Vec<CompressorBlowdownOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_blowdown_overrides = query_as!(
            CompressorBlowdownOverride,
            "SELECT * FROM compressor_blowdown_override WHERE compressor_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_blowdown_overrides
            .sort_by_key(|compressor_blowdown_override| compressor_blowdown_override.compressor_id);

        let compressor_blowdown_overrides_by_compressor = compressor_blowdown_overrides
            .into_iter()
            .group_by(|compressor_blowdown_override| compressor_blowdown_override.compressor_id)
            .into_iter()
            .map(|(compressor_id, group)| (compressor_id, group.collect()))
            .collect();

        Ok(compressor_blowdown_overrides_by_compressor)
    }
}
