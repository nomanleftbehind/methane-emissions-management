use crate::graphql::models::pneumatic_device::non_level_controller::NonLevelControllerMonthMethaneEmissionOverride;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct NonLevelControllerMonthMethaneEmissionOverrideLoader {
    pool: Data<PgPool>,
}

impl NonLevelControllerMonthMethaneEmissionOverrideLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for NonLevelControllerMonthMethaneEmissionOverrideLoader {
    type Value = NonLevelControllerMonthMethaneEmissionOverride;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let non_level_controller_month_methane_emission_overrides = query_as!(
            NonLevelControllerMonthMethaneEmissionOverride,
            "SELECT * FROM non_level_controller_month_methane_emission_override WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|non_level_controller_month_methane_emission_override| {
            (
                non_level_controller_month_methane_emission_override.id,
                non_level_controller_month_methane_emission_override,
            )
        })
        .collect();

        Ok(non_level_controller_month_methane_emission_overrides)
    }
}

pub struct CreatedNonLevelControllerMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl CreatedNonLevelControllerMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedNonLevelControllerMonthMethaneEmissionOverridesLoader {
    type Value = Vec<NonLevelControllerMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut non_level_controller_month_methane_emission_overrides = query_as!(
            NonLevelControllerMonthMethaneEmissionOverride,
            "SELECT * FROM non_level_controller_month_methane_emission_override WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        non_level_controller_month_methane_emission_overrides.sort_by_key(
            |non_level_controller_month_methane_emission_override| {
                non_level_controller_month_methane_emission_override.created_by_id
            },
        );

        let created_non_level_controller_month_methane_emission_overrides =
            non_level_controller_month_methane_emission_overrides
                .into_iter()
                .group_by(|non_level_controller_month_methane_emission_override| {
                    non_level_controller_month_methane_emission_override.created_by_id
                })
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(created_non_level_controller_month_methane_emission_overrides)
    }
}

pub struct UpdatedNonLevelControllerMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl UpdatedNonLevelControllerMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedNonLevelControllerMonthMethaneEmissionOverridesLoader {
    type Value = Vec<NonLevelControllerMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut non_level_controller_month_methane_emission_overrides = query_as!(
            NonLevelControllerMonthMethaneEmissionOverride,
            "SELECT * FROM non_level_controller_month_methane_emission_override WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        non_level_controller_month_methane_emission_overrides.sort_by_key(
            |non_level_controller_month_methane_emission_override| {
                non_level_controller_month_methane_emission_override.updated_by_id
            },
        );

        let updated_non_level_controller_month_methane_emission_overrides =
            non_level_controller_month_methane_emission_overrides
                .into_iter()
                .group_by(|non_level_controller_month_methane_emission_override| {
                    non_level_controller_month_methane_emission_override.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(updated_non_level_controller_month_methane_emission_overrides)
    }
}

pub struct NonLevelControllerMonthMethaneEmissionOverridesByNonLevelControllerLoader {
    pool: Data<PgPool>,
}

impl NonLevelControllerMonthMethaneEmissionOverridesByNonLevelControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for NonLevelControllerMonthMethaneEmissionOverridesByNonLevelControllerLoader {
    type Value = Vec<NonLevelControllerMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut non_level_controller_month_methane_emission_overrides = query_as!(
            NonLevelControllerMonthMethaneEmissionOverride,
            "SELECT * FROM non_level_controller_month_methane_emission_override WHERE non_level_controller_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        non_level_controller_month_methane_emission_overrides.sort_by_key(
            |non_level_controller_month_methane_emission_override| {
                non_level_controller_month_methane_emission_override.non_level_controller_id
            },
        );

        let non_level_controller_month_methane_emission_overrides =
            non_level_controller_month_methane_emission_overrides
                .into_iter()
                .group_by(|non_level_controller_month_methane_emission_override| {
                    non_level_controller_month_methane_emission_override.non_level_controller_id
                })
                .into_iter()
                .map(|(non_level_controller_id, group)| (non_level_controller_id, group.collect()))
                .collect();

        Ok(non_level_controller_month_methane_emission_overrides)
    }
}
