use crate::graphql::models::routine::compressor_seal::CompressorControlDeviceInactivity;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct CompressorControlDeviceInactivityLoader {
    pool: Data<PgPool>,
}

impl CompressorControlDeviceInactivityLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorControlDeviceInactivityLoader {
    type Value = CompressorControlDeviceInactivity;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let compressor_control_device_inactivities = query_as!(
            CompressorControlDeviceInactivity,
            r#"
            SELECT
            id, compressor_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_control_device_inactivity
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|compressor_control_device_inactivity| (compressor_control_device_inactivity.id, compressor_control_device_inactivity))
        .collect();

        Ok(compressor_control_device_inactivities)
    }
}

pub struct CreatedCompressorControlDeviceInactivitiesLoader {
    pool: Data<PgPool>,
}

impl CreatedCompressorControlDeviceInactivitiesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedCompressorControlDeviceInactivitiesLoader {
    type Value = Vec<CompressorControlDeviceInactivity>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_control_device_inactivities = query_as!(
            CompressorControlDeviceInactivity,
            r#"
            SELECT
            id, compressor_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_control_device_inactivity
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_control_device_inactivities.sort_by_key(
            |compressor_control_device_inactivity| {
                compressor_control_device_inactivity.created_by_id
            },
        );

        let compressor_control_device_inactivities = compressor_control_device_inactivities
            .into_iter()
            .group_by(|compressor_control_device_inactivity| {
                compressor_control_device_inactivity.created_by_id
            })
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(compressor_control_device_inactivities)
    }
}

pub struct UpdatedCompressorControlDeviceInactivitiesLoader {
    pool: Data<PgPool>,
}

impl UpdatedCompressorControlDeviceInactivitiesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedCompressorControlDeviceInactivitiesLoader {
    type Value = Vec<CompressorControlDeviceInactivity>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_control_device_inactivities = query_as!(
            CompressorControlDeviceInactivity,
            r#"
            SELECT
            id, compressor_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_control_device_inactivity
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_control_device_inactivities.sort_by_key(
            |compressor_control_device_inactivity| {
                compressor_control_device_inactivity.updated_by_id
            },
        );

        let compressor_control_device_inactivities = compressor_control_device_inactivities
            .into_iter()
            .group_by(|compressor_control_device_inactivity| {
                compressor_control_device_inactivity.updated_by_id
            })
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(compressor_control_device_inactivities)
    }
}

pub struct CompressorControlDeviceInactivitiesByCompressorControlledCharacterizationLoader {
    pool: Data<PgPool>,
}

impl CompressorControlDeviceInactivitiesByCompressorControlledCharacterizationLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid>
    for CompressorControlDeviceInactivitiesByCompressorControlledCharacterizationLoader
{
    type Value = Vec<CompressorControlDeviceInactivity>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_control_device_inactivities = query_as!(
            CompressorControlDeviceInactivity,
            r#"
            SELECT
            id, compressor_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_control_device_inactivity
            WHERE compressor_controlled_characterization_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_control_device_inactivities.sort_by_key(
            |compressor_control_device_inactivity| {
                compressor_control_device_inactivity.compressor_controlled_characterization_id
            },
        );

        let compressor_control_device_inactivities = compressor_control_device_inactivities
            .into_iter()
            .group_by(|compressor_control_device_inactivity| {
                compressor_control_device_inactivity.compressor_controlled_characterization_id
            })
            .into_iter()
            .map(|(compressor_controlled_characterization_id, group)| {
                (compressor_controlled_characterization_id, group.collect())
            })
            .collect();

        Ok(compressor_control_device_inactivities)
    }
}
