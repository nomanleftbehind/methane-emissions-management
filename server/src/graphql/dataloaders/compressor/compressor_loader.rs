use crate::graphql::models::compressor::Compressor;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct CompressorLoader {
    pool: Data<PgPool>,
}

impl CompressorLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorLoader {
    type Value = Compressor;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let compressors = query_as!(
            Compressor,
            r#"
            SELECT
            id, site_id, fdc_rec_id, type as "type: _", controlled, name, serial_number, power, throw_count, install_date, remove_date, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|compressor| (compressor.id, compressor))
        .collect();

        Ok(compressors)
    }
}

pub struct CreatedCompressorsLoader {
    pool: Data<PgPool>,
}

impl CreatedCompressorsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedCompressorsLoader {
    type Value = Vec<Compressor>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressors = sqlx::query_as!(
            Compressor,
            r#"
            SELECT
            id, site_id, fdc_rec_id, type as "type: _", controlled, name, serial_number, power, throw_count, install_date, remove_date, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        compressors.sort_by_key(|compressor| compressor.created_by_id);

        let created_compressors = compressors
            .into_iter()
            .group_by(|compressor| compressor.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_compressors)
    }
}

pub struct UpdatedCompressorsLoader {
    pool: Data<PgPool>,
}

impl UpdatedCompressorsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedCompressorsLoader {
    type Value = Vec<Compressor>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressors = sqlx::query_as!(
            Compressor,
            r#"
            SELECT
            id, site_id, fdc_rec_id, type as "type: _", controlled, name, serial_number, power, throw_count, install_date, remove_date, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        compressors.sort_by_key(|compressor| compressor.updated_by_id);

        let updated_compressors = compressors
            .into_iter()
            .group_by(|compressor| compressor.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_compressors)
    }
}

pub struct SiteCompressorsLoader {
    pool: Data<PgPool>,
}

impl SiteCompressorsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for SiteCompressorsLoader {
    type Value = Vec<Compressor>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressors = sqlx::query_as!(
            Compressor,
            r#"
            SELECT
            id, site_id, fdc_rec_id, type as "type: _", controlled, name, serial_number, power, throw_count, install_date, remove_date, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor
            WHERE site_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        compressors.sort_by_key(|compressor| compressor.facility_id);

        let facility_compressors = compressors
            .into_iter()
            .group_by(|compressor| compressor.facility_id)
            .into_iter()
            .map(|(facility_id, group)| (facility_id, group.collect()))
            .collect();

        Ok(facility_compressors)
    }
}
