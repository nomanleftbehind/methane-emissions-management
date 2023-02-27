use crate::graphql::models::TankFarmMonthVentOverride;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct TankFarmMonthVentOverrideLoader {
    pool: Data<PgPool>,
}

impl TankFarmMonthVentOverrideLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankFarmMonthVentOverrideLoader {
    type Value = TankFarmMonthVentOverride;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tank_farm_month_vent_override_overrides = sqlx::query_as!(
            TankFarmMonthVentOverride,
            r#"SELECT * FROM tank_farm_month_vent_override WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|tank_farm_month_vent_override| {
            (
                tank_farm_month_vent_override.id,
                tank_farm_month_vent_override,
            )
        })
        .collect();

        Ok(tank_farm_month_vent_override_overrides)
    }
}

pub struct CreatedTankFarmMonthVentOverridesLoader {
    pool: Data<PgPool>,
}

impl CreatedTankFarmMonthVentOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedTankFarmMonthVentOverridesLoader {
    type Value = Vec<TankFarmMonthVentOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farm_month_vent_override_overrides = sqlx::query_as!(
            TankFarmMonthVentOverride,
            r#"SELECT * FROM tank_farm_month_vent_override WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_farm_month_vent_override_overrides.sort_by_key(|tank_farm_month_vent_override| {
            tank_farm_month_vent_override.created_by_id
        });

        let created_tank_farm_month_vent_override_overrides =
            tank_farm_month_vent_override_overrides
                .into_iter()
                .group_by(|tank_farm_month_vent_override| {
                    tank_farm_month_vent_override.created_by_id
                })
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(created_tank_farm_month_vent_override_overrides)
    }
}

pub struct UpdatedTankFarmMonthVentOverridesLoader {
    pool: Data<PgPool>,
}

impl UpdatedTankFarmMonthVentOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedTankFarmMonthVentOverridesLoader {
    type Value = Vec<TankFarmMonthVentOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farm_month_vent_override_overrides = sqlx::query_as!(
            TankFarmMonthVentOverride,
            r#"SELECT * FROM tank_farm_month_vent_override WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_farm_month_vent_override_overrides.sort_by_key(|tank_farm_month_vent_override| {
            tank_farm_month_vent_override.updated_by_id
        });

        let updated_tank_farm_month_vent_override_overrides =
            tank_farm_month_vent_override_overrides
                .into_iter()
                .group_by(|tank_farm_month_vent_override| {
                    tank_farm_month_vent_override.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(updated_tank_farm_month_vent_override_overrides)
    }
}

pub struct TankFarmMonthVentOverridesByTankFarmLoader {
    pool: Data<PgPool>,
}

impl TankFarmMonthVentOverridesByTankFarmLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankFarmMonthVentOverridesByTankFarmLoader {
    type Value = Vec<TankFarmMonthVentOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farm_month_vent_override_overrides = sqlx::query_as!(
            TankFarmMonthVentOverride,
            r#"SELECT * FROM tank_farm_month_vent_override WHERE tank_farm_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_farm_month_vent_override_overrides.sort_by_key(|tank_farm_month_vent_override| {
            tank_farm_month_vent_override.tank_farm_id
        });

        let tank_farm_month_vent_override_overrides_by_tank_farm =
            tank_farm_month_vent_override_overrides
                .into_iter()
                .group_by(|tank_farm_month_vent_override| {
                    tank_farm_month_vent_override.tank_farm_id
                })
                .into_iter()
                .map(|(tank_farm_id, group)| (tank_farm_id, group.collect()))
                .collect();

        Ok(tank_farm_month_vent_override_overrides_by_tank_farm)
    }
}
