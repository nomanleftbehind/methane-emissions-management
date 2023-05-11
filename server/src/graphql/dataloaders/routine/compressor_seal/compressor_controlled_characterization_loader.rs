use crate::graphql::models::routine::compressor_seal::CompressorControlledCharacterization;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct CompressorControlledCharacterizationLoader {
    pool: Data<PgPool>,
}

impl CompressorControlledCharacterizationLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorControlledCharacterizationLoader {
    type Value = CompressorControlledCharacterization;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let compressor_controlled_characterizations = query_as!(
            CompressorControlledCharacterization,
            r#"
            SELECT
            id, compressor_id, date, controlled_characterization as "controlled_characterization: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_controlled_characterization
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|compressor_controlled_characterization| (compressor_controlled_characterization.id, compressor_controlled_characterization))
        .collect();

        Ok(compressor_controlled_characterizations)
    }
}

pub struct CreatedCompressorControlledCharacterizationsLoader {
    pool: Data<PgPool>,
}

impl CreatedCompressorControlledCharacterizationsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedCompressorControlledCharacterizationsLoader {
    type Value = Vec<CompressorControlledCharacterization>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_controlled_characterizations = query_as!(
            CompressorControlledCharacterization,
            r#"
            SELECT
            id, compressor_id, date, controlled_characterization as "controlled_characterization: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_controlled_characterization
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_controlled_characterizations.sort_by_key(
            |compressor_controlled_characterization| {
                compressor_controlled_characterization.created_by_id
            },
        );

        let compressor_controlled_characterizations = compressor_controlled_characterizations
            .into_iter()
            .group_by(|cf| cf.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(compressor_controlled_characterizations)
    }
}

pub struct UpdatedCompressorControlledCharacterizationsLoader {
    pool: Data<PgPool>,
}

impl UpdatedCompressorControlledCharacterizationsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedCompressorControlledCharacterizationsLoader {
    type Value = Vec<CompressorControlledCharacterization>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_controlled_characterizations = query_as!(
            CompressorControlledCharacterization,
            r#"
            SELECT
            id, compressor_id, date, controlled_characterization as "controlled_characterization: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_controlled_characterization
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_controlled_characterizations.sort_by_key(
            |compressor_controlled_characterization| {
                compressor_controlled_characterization.updated_by_id
            },
        );

        let compressor_controlled_characterizations = compressor_controlled_characterizations
            .into_iter()
            .group_by(|compressor_controlled_characterization| {
                compressor_controlled_characterization.updated_by_id
            })
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(compressor_controlled_characterizations)
    }
}

pub struct CompressorControlledCharacterizationsByCompressorLoader {
    pool: Data<PgPool>,
}

impl CompressorControlledCharacterizationsByCompressorLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorControlledCharacterizationsByCompressorLoader {
    type Value = Vec<CompressorControlledCharacterization>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_controlled_characterizations = sqlx::query_as!(
            CompressorControlledCharacterization,
            r#"
            SELECT
            id, compressor_id, date, controlled_characterization as "controlled_characterization: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_controlled_characterization
            WHERE compressor_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_controlled_characterizations.sort_by_key(
            |compressor_controlled_characterization| {
                compressor_controlled_characterization.compressor_id
            },
        );

        let compressor_controlled_characterizations = compressor_controlled_characterizations
            .into_iter()
            .group_by(|compressor_controlled_characterization| {
                compressor_controlled_characterization.compressor_id
            })
            .into_iter()
            .map(|(compressor_id, group)| (compressor_id, group.collect()))
            .collect();

        Ok(compressor_controlled_characterizations)
    }
}
