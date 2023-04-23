use crate::graphql::models::defined_vent_gas::tank::TankMonthMethaneEmissionOverride;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct TankMonthMethaneEmissionOverrideLoader {
    pool: Data<PgPool>,
}

impl TankMonthMethaneEmissionOverrideLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankMonthMethaneEmissionOverrideLoader {
    type Value = TankMonthMethaneEmissionOverride;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tank_month_methane_emission_overrides = query_as!(
            TankMonthMethaneEmissionOverride,
            r#"SELECT * FROM tank_month_methane_emission_override WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|tank_month_methane_emission_override| {
            (
                tank_month_methane_emission_override.id,
                tank_month_methane_emission_override,
            )
        })
        .collect();

        Ok(tank_month_methane_emission_overrides)
    }
}

pub struct CreatedTankMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl CreatedTankMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedTankMonthMethaneEmissionOverridesLoader {
    type Value = Vec<TankMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_month_methane_emission_overrides = query_as!(
            TankMonthMethaneEmissionOverride,
            r#"SELECT * FROM tank_month_methane_emission_override WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_month_methane_emission_overrides.sort_by_key(|tank_month_methane_emission_override| {
            tank_month_methane_emission_override.created_by_id
        });

        let tank_month_methane_emission_overrides = tank_month_methane_emission_overrides
            .into_iter()
            .group_by(|tank_month_methane_emission_override| {
                tank_month_methane_emission_override.created_by_id
            })
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(tank_month_methane_emission_overrides)
    }
}

pub struct UpdatedTankMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl UpdatedTankMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedTankMonthMethaneEmissionOverridesLoader {
    type Value = Vec<TankMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_month_methane_emission_overrides = query_as!(
            TankMonthMethaneEmissionOverride,
            r#"SELECT * FROM tank_month_methane_emission_override WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_month_methane_emission_overrides.sort_by_key(|tank_month_methane_emission_override| {
            tank_month_methane_emission_override.updated_by_id
        });

        let tank_month_methane_emission_overrides = tank_month_methane_emission_overrides
            .into_iter()
            .group_by(|tank_month_methane_emission_override| {
                tank_month_methane_emission_override.updated_by_id
            })
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(tank_month_methane_emission_overrides)
    }
}

pub struct TankMonthMethaneEmissionOverridesByTankLoader {
    pool: Data<PgPool>,
}

impl TankMonthMethaneEmissionOverridesByTankLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankMonthMethaneEmissionOverridesByTankLoader {
    type Value = Vec<TankMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_month_methane_emission_overrides = query_as!(
            TankMonthMethaneEmissionOverride,
            r#"SELECT * FROM tank_month_methane_emission_override WHERE tank_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_month_methane_emission_overrides.sort_by_key(|tank_month_methane_emission_override| {
            tank_month_methane_emission_override.tank_id
        });

        let tank_month_methane_emission_overrides = tank_month_methane_emission_overrides
            .into_iter()
            .group_by(|tank_month_methane_emission_override| {
                tank_month_methane_emission_override.tank_id
            })
            .into_iter()
            .map(|(tank_id, group)| (tank_id, group.collect()))
            .collect();

        Ok(tank_month_methane_emission_overrides)
    }
}
