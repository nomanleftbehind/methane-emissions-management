use crate::graphql::models::routine::pneumatic_device::level_controller::LevelControllerEmissionRate;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct LevelControllerEmissionRateLoader {
    pool: Data<PgPool>,
}

impl LevelControllerEmissionRateLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for LevelControllerEmissionRateLoader {
    type Value = LevelControllerEmissionRate;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let level_controller_emission_rates = query_as!(
            LevelControllerEmissionRate,
            "SELECT * FROM level_controller_emission_rate WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|level_controller_emission_rate| {
            (
                level_controller_emission_rate.id,
                level_controller_emission_rate,
            )
        })
        .collect();

        Ok(level_controller_emission_rates)
    }
}

pub struct CreatedLevelControllerEmissionRatesLoader {
    pool: Data<PgPool>,
}

impl CreatedLevelControllerEmissionRatesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedLevelControllerEmissionRatesLoader {
    type Value = Vec<LevelControllerEmissionRate>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_emission_rates = query_as!(
            LevelControllerEmissionRate,
            "SELECT * FROM level_controller_emission_rate WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_emission_rates.sort_by_key(|level_controller_emission_rate| {
            level_controller_emission_rate.created_by_id
        });

        let level_controller_emission_rates = level_controller_emission_rates
            .into_iter()
            .group_by(|level_controller_emission_rate| level_controller_emission_rate.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(level_controller_emission_rates)
    }
}

pub struct UpdatedLevelControllerEmissionRatesLoader {
    pool: Data<PgPool>,
}

impl UpdatedLevelControllerEmissionRatesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedLevelControllerEmissionRatesLoader {
    type Value = Vec<LevelControllerEmissionRate>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_emission_rates = query_as!(
            LevelControllerEmissionRate,
            "SELECT * FROM level_controller_emission_rate WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_emission_rates
            .sort_by_key(|level_controller| level_controller.updated_by_id);

        let level_controller_emission_rates = level_controller_emission_rates
            .into_iter()
            .group_by(|level_controller| level_controller.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(level_controller_emission_rates)
    }
}

pub struct LevelControllerEmissionRatesByLevelControllerLoader {
    pool: Data<PgPool>,
}

impl LevelControllerEmissionRatesByLevelControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for LevelControllerEmissionRatesByLevelControllerLoader {
    type Value = Vec<LevelControllerEmissionRate>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_emission_rate = query_as!(
            LevelControllerEmissionRate,
            "SELECT * FROM level_controller_emission_rate WHERE level_controller_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_emission_rate.sort_by_key(|level_controller_emission_rate| {
            level_controller_emission_rate.level_controller_id
        });

        let level_controller_emission_rates = level_controller_emission_rate
            .into_iter()
            .group_by(|level_controller_emission_rate| {
                level_controller_emission_rate.level_controller_id
            })
            .into_iter()
            .map(|(level_controller_id, group)| (level_controller_id, group.collect()))
            .collect();

        Ok(level_controller_emission_rates)
    }
}
