use crate::graphql::models::routine::pneumatic_device::level_controller::LevelControllerMonthMethaneEmissionOverride;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct LevelControllerMonthMethaneEmissionOverrideLoader {
    pool: Data<PgPool>,
}

impl LevelControllerMonthMethaneEmissionOverrideLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for LevelControllerMonthMethaneEmissionOverrideLoader {
    type Value = LevelControllerMonthMethaneEmissionOverride;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let level_controller_month_methane_emission_overrides = query_as!(
            LevelControllerMonthMethaneEmissionOverride,
            "SELECT * FROM level_controller_month_methane_emission_override WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|level_controller_month_methane_emission_override| {
            (
                level_controller_month_methane_emission_override.id,
                level_controller_month_methane_emission_override,
            )
        })
        .collect();

        Ok(level_controller_month_methane_emission_overrides)
    }
}

pub struct CreatedLevelControllerMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl CreatedLevelControllerMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedLevelControllerMonthMethaneEmissionOverridesLoader {
    type Value = Vec<LevelControllerMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_month_methane_emission_overrides = query_as!(
            LevelControllerMonthMethaneEmissionOverride,
            "SELECT * FROM level_controller_month_methane_emission_override WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_month_methane_emission_overrides.sort_by_key(
            |level_controller_month_methane_emission_override| {
                level_controller_month_methane_emission_override.created_by_id
            },
        );

        let created_level_controller_month_methane_emission_overrides =
            level_controller_month_methane_emission_overrides
                .into_iter()
                .group_by(|level_controller_month_methane_emission_override| {
                    level_controller_month_methane_emission_override.created_by_id
                })
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(created_level_controller_month_methane_emission_overrides)
    }
}

pub struct UpdatedLevelControllerMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl UpdatedLevelControllerMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedLevelControllerMonthMethaneEmissionOverridesLoader {
    type Value = Vec<LevelControllerMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_month_methane_emission_overrides = query_as!(
            LevelControllerMonthMethaneEmissionOverride,
            "SELECT * FROM level_controller_month_methane_emission_override WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_month_methane_emission_overrides.sort_by_key(
            |level_controller_month_methane_emission_override| {
                level_controller_month_methane_emission_override.updated_by_id
            },
        );

        let updated_level_controller_month_methane_emission_overrides =
            level_controller_month_methane_emission_overrides
                .into_iter()
                .group_by(|level_controller_month_methane_emission_override| {
                    level_controller_month_methane_emission_override.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(updated_level_controller_month_methane_emission_overrides)
    }
}

pub struct LevelControllerMonthMethaneEmissionOverridesByLevelControllerLoader {
    pool: Data<PgPool>,
}

impl LevelControllerMonthMethaneEmissionOverridesByLevelControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for LevelControllerMonthMethaneEmissionOverridesByLevelControllerLoader {
    type Value = Vec<LevelControllerMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_month_methane_emission_overrides = query_as!(
            LevelControllerMonthMethaneEmissionOverride,
            "SELECT * FROM level_controller_month_methane_emission_override WHERE level_controller_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_month_methane_emission_overrides.sort_by_key(
            |level_controller_month_methane_emission_override| {
                level_controller_month_methane_emission_override.level_controller_id
            },
        );

        let level_controller_month_methane_emission_overrides =
            level_controller_month_methane_emission_overrides
                .into_iter()
                .group_by(|level_controller_month_methane_emission_override| {
                    level_controller_month_methane_emission_override.level_controller_id
                })
                .into_iter()
                .map(|(level_controller_id, group)| (level_controller_id, group.collect()))
                .collect();

        Ok(level_controller_month_methane_emission_overrides)
    }
}
