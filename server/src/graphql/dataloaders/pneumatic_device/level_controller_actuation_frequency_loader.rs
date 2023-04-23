use crate::graphql::models::pneumatic_device::LevelControllerActuationFrequency;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct LevelControllerActuationFrequencyLoader {
    pool: Data<PgPool>,
}

impl LevelControllerActuationFrequencyLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for LevelControllerActuationFrequencyLoader {
    type Value = LevelControllerActuationFrequency;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let level_controller_actuation_frequencies = query_as!(
            LevelControllerActuationFrequency,
            "SELECT * FROM level_controller_actuation_frequency WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|level_controller_actuation_frequency| {
            (
                level_controller_actuation_frequency.id,
                level_controller_actuation_frequency,
            )
        })
        .collect();

        Ok(level_controller_actuation_frequencies)
    }
}

pub struct CreatedLevelControllerActuationFrequenciesLoader {
    pool: Data<PgPool>,
}

impl CreatedLevelControllerActuationFrequenciesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedLevelControllerActuationFrequenciesLoader {
    type Value = Vec<LevelControllerActuationFrequency>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_actuation_frequencies = query_as!(
            LevelControllerActuationFrequency,
            "SELECT * FROM level_controller_actuation_frequency WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_actuation_frequencies.sort_by_key(
            |level_controller_actuation_frequency| {
                level_controller_actuation_frequency.created_by_id
            },
        );

        let level_controller_actuation_frequencies = level_controller_actuation_frequencies
            .into_iter()
            .group_by(|level_controller_actuation_frequency| {
                level_controller_actuation_frequency.created_by_id
            })
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(level_controller_actuation_frequencies)
    }
}

pub struct UpdatedLevelControllerActuationFrequenciesLoader {
    pool: Data<PgPool>,
}

impl UpdatedLevelControllerActuationFrequenciesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedLevelControllerActuationFrequenciesLoader {
    type Value = Vec<LevelControllerActuationFrequency>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_actuation_frequencies = query_as!(
            LevelControllerActuationFrequency,
            "SELECT * FROM level_controller_actuation_frequency WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_actuation_frequencies
            .sort_by_key(|pneumatic_device| pneumatic_device.updated_by_id);

        let level_controller_actuation_frequencies = level_controller_actuation_frequencies
            .into_iter()
            .group_by(|pneumatic_device| pneumatic_device.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(level_controller_actuation_frequencies)
    }
}

pub struct LevelControllerActuationFrequenciesByLevelControllerLoader {
    pool: Data<PgPool>,
}

impl LevelControllerActuationFrequenciesByLevelControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for LevelControllerActuationFrequenciesByLevelControllerLoader {
    type Value = Vec<LevelControllerActuationFrequency>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_actuation_frequency = query_as!(
            LevelControllerActuationFrequency,
            "SELECT * FROM level_controller_actuation_frequency WHERE pneumatic_device_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_actuation_frequency.sort_by_key(|level_controller_actuation_frequency| {
            level_controller_actuation_frequency.pneumatic_device_id
        });

        let level_controller_actuation_frequencies = level_controller_actuation_frequency
            .into_iter()
            .group_by(|level_controller_actuation_frequency| {
                level_controller_actuation_frequency.pneumatic_device_id
            })
            .into_iter()
            .map(|(pneumatic_device_id, group)| (pneumatic_device_id, group.collect()))
            .collect();

        Ok(level_controller_actuation_frequencies)
    }
}
