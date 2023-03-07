use crate::graphql::models::TankFarmVentFactorCalculated;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct TankFarmVentFactorCalculatedLoader {
    pool: Data<PgPool>,
}

impl TankFarmVentFactorCalculatedLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankFarmVentFactorCalculatedLoader {
    type Value = TankFarmVentFactorCalculated;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tank_farm_vent_factors_calculated = sqlx::query_as!(
            TankFarmVentFactorCalculated,
            r#"SELECT * FROM tank_farm_vent_factors_calculated WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|tank_farm_vent_factor_calculated| {
            (
                tank_farm_vent_factor_calculated.id,
                tank_farm_vent_factor_calculated,
            )
        })
        .collect();

        Ok(tank_farm_vent_factors_calculated)
    }
}

pub struct CreatedTankFarmVentFactorsCalculatedLoader {
    pool: Data<PgPool>,
}

impl CreatedTankFarmVentFactorsCalculatedLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedTankFarmVentFactorsCalculatedLoader {
    type Value = Vec<TankFarmVentFactorCalculated>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farm_vent_factors_calculated = sqlx::query_as!(
            TankFarmVentFactorCalculated,
            r#"SELECT * FROM tank_farm_vent_factors_calculated WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_farm_vent_factors_calculated.sort_by_key(|tank_farm_vent_factor_calculated| {
            tank_farm_vent_factor_calculated.created_by_id
        });

        let created_tank_farm_vent_factors_calculated = tank_farm_vent_factors_calculated
            .into_iter()
            .group_by(|tank_farm_vent_factor_calculated| {
                tank_farm_vent_factor_calculated.created_by_id
            })
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_tank_farm_vent_factors_calculated)
    }
}

pub struct UpdatedTankFarmVentFactorsCalculatedLoader {
    pool: Data<PgPool>,
}

impl UpdatedTankFarmVentFactorsCalculatedLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedTankFarmVentFactorsCalculatedLoader {
    type Value = Vec<TankFarmVentFactorCalculated>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farm_vent_factors_calculated = sqlx::query_as!(
            TankFarmVentFactorCalculated,
            r#"SELECT * FROM tank_farm_vent_factors_calculated WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_farm_vent_factors_calculated.sort_by_key(|tank_farm_vent_factor_calculated| {
            tank_farm_vent_factor_calculated.updated_by_id
        });

        let updated_tank_farm_vent_factors_calculated = tank_farm_vent_factors_calculated
            .into_iter()
            .group_by(|tank_farm_vent_factor_calculated| {
                tank_farm_vent_factor_calculated.updated_by_id
            })
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_tank_farm_vent_factors_calculated)
    }
}

pub struct TankFarmVentFactorsCalculatedByTankFarmLoader {
    pool: Data<PgPool>,
}

impl TankFarmVentFactorsCalculatedByTankFarmLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankFarmVentFactorsCalculatedByTankFarmLoader {
    type Value = Vec<TankFarmVentFactorCalculated>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farm_vent_factors_calculated = sqlx::query_as!(
            TankFarmVentFactorCalculated,
            r#"SELECT * FROM tank_farm_vent_factors_calculated WHERE tank_farm_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_farm_vent_factors_calculated.sort_by_key(|tank_farm_vent_factor_calculated| {
            tank_farm_vent_factor_calculated.tank_farm_id
        });

        let tank_farm_vent_factors_calculated_by_tank_farm = tank_farm_vent_factors_calculated
            .into_iter()
            .group_by(|tank_farm_vent_factor_calculated| {
                tank_farm_vent_factor_calculated.tank_farm_id
            })
            .into_iter()
            .map(|(tank_farm_id, group)| (tank_farm_id, group.collect()))
            .collect();

        Ok(tank_farm_vent_factors_calculated_by_tank_farm)
    }
}
