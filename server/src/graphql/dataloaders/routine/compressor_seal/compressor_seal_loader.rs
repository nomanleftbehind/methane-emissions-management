use crate::graphql::models::routine::compressor_seal::CompressorSeal;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct CompressorSealLoader {
    pool: Data<PgPool>,
}

impl CompressorSealLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorSealLoader {
    type Value = CompressorSeal;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let compressor_seals = query_as!(
            CompressorSeal,
            r#"
            SELECT
            id, type as "type: _", description, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_seal
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|compressor_seal| (compressor_seal.id, compressor_seal))
        .collect();

        Ok(compressor_seals)
    }
}

pub struct CreatedCompressorSealsLoader {
    pool: Data<PgPool>,
}

impl CreatedCompressorSealsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedCompressorSealsLoader {
    type Value = Vec<CompressorSeal>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_seals = query_as!(
            CompressorSeal,
            r#"
            SELECT
            id, type as "type: _", description, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_seal
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        compressor_seals.sort_by_key(|compressor_seal| compressor_seal.created_by_id);

        let compressor_seals = compressor_seals
            .into_iter()
            .group_by(|compressor_seal| compressor_seal.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(compressor_seals)
    }
}

pub struct UpdatedCompressorSealsLoader {
    pool: Data<PgPool>,
}

impl UpdatedCompressorSealsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedCompressorSealsLoader {
    type Value = Vec<CompressorSeal>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_seals = query_as!(
            CompressorSeal,
            r#"
            SELECT
            id, type as "type: _", description, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_seal
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        compressor_seals.sort_by_key(|compressor_seal| compressor_seal.updated_by_id);

        let compressor_seals = compressor_seals
            .into_iter()
            .group_by(|compressor_seal| compressor_seal.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(compressor_seals)
    }
}
