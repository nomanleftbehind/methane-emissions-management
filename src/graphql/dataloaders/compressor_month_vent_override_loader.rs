use crate::graphql::models::CompressorMonthVentOverride;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CompressorMonthVentOverrideLoader {
    pool: Data<PgPool>,
}

impl CompressorMonthVentOverrideLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorMonthVentOverrideLoader {
    type Value = CompressorMonthVentOverride;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let compressor_month_vent_override_overrides = sqlx::query_as!(
            CompressorMonthVentOverride,
            r#"SELECT * FROM compressor_month_vent_override WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|compressor_month_vent_override| {
            (
                compressor_month_vent_override.id,
                compressor_month_vent_override,
            )
        })
        .collect();

        Ok(compressor_month_vent_override_overrides)
    }
}

pub struct CreatedCompressorMonthVentOverridesLoader {
    pool: Data<PgPool>,
}

impl CreatedCompressorMonthVentOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedCompressorMonthVentOverridesLoader {
    type Value = Vec<CompressorMonthVentOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_month_vent_override_overrides = sqlx::query_as!(
            CompressorMonthVentOverride,
            r#"SELECT * FROM compressor_month_vent_override WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_month_vent_override_overrides.sort_by_key(|compressor_month_vent_override| {
            compressor_month_vent_override.created_by_id
        });

        let created_compressor_month_vent_override_overrides =
            compressor_month_vent_override_overrides
                .into_iter()
                .group_by(|compressor_month_vent_override| {
                    compressor_month_vent_override.created_by_id
                })
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(created_compressor_month_vent_override_overrides)
    }
}

pub struct UpdatedCompressorMonthVentOverridesLoader {
    pool: Data<PgPool>,
}

impl UpdatedCompressorMonthVentOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedCompressorMonthVentOverridesLoader {
    type Value = Vec<CompressorMonthVentOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_month_vent_override_overrides = sqlx::query_as!(
            CompressorMonthVentOverride,
            r#"SELECT * FROM compressor_month_vent_override WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_month_vent_override_overrides.sort_by_key(|compressor_month_vent_override| {
            compressor_month_vent_override.updated_by_id
        });

        let updated_compressor_month_vent_override_overrides =
            compressor_month_vent_override_overrides
                .into_iter()
                .group_by(|compressor_month_vent_override| {
                    compressor_month_vent_override.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(updated_compressor_month_vent_override_overrides)
    }
}

pub struct CompressorMonthVentOverridesByCompressorLoader {
    pool: Data<PgPool>,
}

impl CompressorMonthVentOverridesByCompressorLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorMonthVentOverridesByCompressorLoader {
    type Value = Vec<CompressorMonthVentOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_month_vent_override_overrides = sqlx::query_as!(
            CompressorMonthVentOverride,
            r#"SELECT * FROM compressor_month_vent_override WHERE compressor_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_month_vent_override_overrides.sort_by_key(|compressor_month_vent_override| {
            compressor_month_vent_override.compressor_id
        });

        let compressor_month_vent_override_overrides_by_compressor =
            compressor_month_vent_override_overrides
                .into_iter()
                .group_by(|compressor_month_vent_override| {
                    compressor_month_vent_override.compressor_id
                })
                .into_iter()
                .map(|(compressor_id, group)| (compressor_id, group.collect()))
                .collect();

        Ok(compressor_month_vent_override_overrides_by_compressor)
    }
}
