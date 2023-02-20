use crate::graphql::models::CompressorMonthVent;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CompressorMonthVentLoader {
    pool: Data<PgPool>,
}

impl CompressorMonthVentLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorMonthVentLoader {
    type Value = CompressorMonthVent;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let compressor_month_vents = sqlx::query_as!(
            CompressorMonthVent,
            "SELECT * FROM compressor_month_vent WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|compressor_month_vent| (compressor_month_vent.id, compressor_month_vent))
        .collect();

        Ok(compressor_month_vents)
    }
}

pub struct CreatedCompressorMonthVentsLoader {
    pool: Data<PgPool>,
}

impl CreatedCompressorMonthVentsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedCompressorMonthVentsLoader {
    type Value = Vec<CompressorMonthVent>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_month_vents = sqlx::query_as!(
            CompressorMonthVent,
            "SELECT * FROM compressor_month_vent WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_month_vents
            .sort_by_key(|compressor_month_vent| compressor_month_vent.created_by_id);

        let created_compressor_month_vents = compressor_month_vents
            .into_iter()
            .group_by(|compressor_month_vent| compressor_month_vent.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_compressor_month_vents)
    }
}

pub struct UpdatedCompressorMonthVentsLoader {
    pool: Data<PgPool>,
}

impl UpdatedCompressorMonthVentsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedCompressorMonthVentsLoader {
    type Value = Vec<CompressorMonthVent>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_month_vents = sqlx::query_as!(
            CompressorMonthVent,
            "SELECT * FROM compressor_month_vent WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_month_vents
            .sort_by_key(|compressor_month_vent| compressor_month_vent.updated_by_id);

        let updated_compressor_month_vents = compressor_month_vents
            .into_iter()
            .group_by(|compressor_month_vent| compressor_month_vent.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_compressor_month_vents)
    }
}

pub struct CompressorMonthVentsByCompressorLoader {
    pool: Data<PgPool>,
}

impl CompressorMonthVentsByCompressorLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorMonthVentsByCompressorLoader {
    type Value = Vec<CompressorMonthVent>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_month_vents = sqlx::query_as!(
            CompressorMonthVent,
            "SELECT * FROM compressor_month_vent WHERE compressor_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_month_vents
            .sort_by_key(|compressor_month_vent| compressor_month_vent.compressor_id);

        let compressor_month_vents_by_compressor = compressor_month_vents
            .into_iter()
            .group_by(|compressor_month_vent| compressor_month_vent.compressor_id)
            .into_iter()
            .map(|(compressor_id, group)| (compressor_id, group.collect()))
            .collect();

        Ok(compressor_month_vents_by_compressor)
    }
}
