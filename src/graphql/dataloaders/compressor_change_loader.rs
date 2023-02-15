use crate::graphql::models::CompressorChange;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CreatedCompressorChangesLoader {
    pool: Data<PgPool>,
}

impl CreatedCompressorChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedCompressorChangesLoader {
    type Value = Vec<CompressorChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_changes = sqlx::query_as!(
            CompressorChange,
            r#"SELECT id, compressor_id, date, calculation_method as "calculation_method: _", number_of_throws, vent_rate_per_hour, created_by_id, created_at, updated_by_id, updated_at FROM compressor_changes WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_changes.sort_by_key(|compressor_change| compressor_change.created_by_id);

        let created_compressor_changes = compressor_changes
            .into_iter()
            .group_by(|compressor_change| compressor_change.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_compressor_changes)
    }
}

pub struct UpdatedCompressorChangesLoader {
    pool: Data<PgPool>,
}

impl UpdatedCompressorChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedCompressorChangesLoader {
    type Value = Vec<CompressorChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_changes = sqlx::query_as!(
            CompressorChange,
            r#"SELECT id, compressor_id, date, calculation_method as "calculation_method: _", number_of_throws, vent_rate_per_hour, created_by_id, created_at, updated_by_id, updated_at FROM compressor_changes WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_changes.sort_by_key(|compressor_change| compressor_change.updated_by_id);

        let updated_compressor_changes = compressor_changes
            .into_iter()
            .group_by(|compressor_change| compressor_change.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_compressor_changes)
    }
}

pub struct CompressorChangeLoader {
    pool: Data<PgPool>,
}

impl CompressorChangeLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorChangeLoader {
    type Value = CompressorChange;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let compressor_changes = sqlx::query_as!(
            CompressorChange,
            r#"SELECT id, compressor_id, date, calculation_method as "calculation_method: _", number_of_throws, vent_rate_per_hour, created_by_id, created_at, updated_by_id, updated_at FROM compressor_changes WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|compressor_change| (compressor_change.id, compressor_change))
        .collect();

        Ok(compressor_changes)
    }
}

pub struct CompressorChangesByCompressorLoader {
    pool: Data<PgPool>,
}

impl CompressorChangesByCompressorLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorChangesByCompressorLoader {
    type Value = Vec<CompressorChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_changes = sqlx::query_as!(
            CompressorChange,
            r#"SELECT id, compressor_id, date, calculation_method as "calculation_method: _", number_of_throws, vent_rate_per_hour, created_by_id, created_at, updated_by_id, updated_at FROM compressor_changes WHERE compressor_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_changes.sort_by_key(|compressor_change| compressor_change.compressor_id);

        let compressor_changes_by_compressor = compressor_changes
            .into_iter()
            .group_by(|compressor_change| compressor_change.compressor_id)
            .into_iter()
            .map(|(compressor_id, group)| (compressor_id, group.collect()))
            .collect();

        Ok(compressor_changes_by_compressor)
    }
}
