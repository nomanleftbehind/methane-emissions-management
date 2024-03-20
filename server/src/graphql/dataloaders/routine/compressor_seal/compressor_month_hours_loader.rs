use crate::graphql::models::routine::compressor_seal::CompressorMonthHours;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct CompressorMonthHoursLoader {
    pool: Data<PgPool>,
}

impl CompressorMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CompressorMonthHoursLoader {
    type Value = CompressorMonthHours;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let compressor_month_hours = query_as!(
            CompressorMonthHours,
            "SELECT * FROM compressor_month_hours WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|compressor_month_hours| (compressor_month_hours.id, compressor_month_hours))
        .collect();

        Ok(compressor_month_hours)
    }
}

pub struct CreatedCompressorMonthHoursLoader {
    pool: Data<PgPool>,
}

impl CreatedCompressorMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedCompressorMonthHoursLoader {
    type Value = Vec<CompressorMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_month_hours = query_as!(
            CompressorMonthHours,
            "SELECT * FROM compressor_month_hours WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_month_hours
            .sort_by_key(|compressor_month_hours| compressor_month_hours.created_by_id);

        let compressor_month_hours = compressor_month_hours
            .into_iter()
            .group_by(|cf| cf.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(compressor_month_hours)
    }
}

pub struct UpdatedCompressorMonthHoursLoader {
    pool: Data<PgPool>,
}

impl UpdatedCompressorMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedCompressorMonthHoursLoader {
    type Value = Vec<CompressorMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_month_hours = query_as!(
            CompressorMonthHours,
            "SELECT * FROM compressor_month_hours WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_month_hours
            .sort_by_key(|compressor_month_hours| compressor_month_hours.updated_by_id);

        let compressor_month_hours = compressor_month_hours
            .into_iter()
            .group_by(|compressor_month_hours| compressor_month_hours.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(compressor_month_hours)
    }
}

pub struct CompressorMonthHoursByCompressorLoader {
    pool: Data<PgPool>,
}

impl CompressorMonthHoursByCompressorLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CompressorMonthHoursByCompressorLoader {
    type Value = Vec<CompressorMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_month_hours = sqlx::query_as!(
            CompressorMonthHours,
            "SELECT * FROM compressor_month_hours WHERE compressor_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_month_hours
            .sort_by_key(|compressor_month_hours| compressor_month_hours.compressor_id);

        let compressor_month_hours = compressor_month_hours
            .into_iter()
            .group_by(|compressor_month_hours| compressor_month_hours.compressor_id)
            .into_iter()
            .map(|(compressor_id, group)| (compressor_id, group.collect()))
            .collect();

        Ok(compressor_month_hours)
    }
}
