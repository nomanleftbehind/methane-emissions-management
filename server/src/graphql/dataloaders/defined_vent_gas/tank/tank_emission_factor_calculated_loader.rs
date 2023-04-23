use crate::graphql::models::defined_vent_gas::tank::TankEmissionFactorCalculated;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct TankEmissionFactorCalculatedLoader {
    pool: Data<PgPool>,
}

impl TankEmissionFactorCalculatedLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankEmissionFactorCalculatedLoader {
    type Value = TankEmissionFactorCalculated;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tank_emission_factor_calculated = query_as!(
            TankEmissionFactorCalculated,
            "SELECT * FROM tank_emission_factor_calculated WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|tank_emission_factor_calculated| {
            (
                tank_emission_factor_calculated.id,
                tank_emission_factor_calculated,
            )
        })
        .collect();

        Ok(tank_emission_factor_calculated)
    }
}

pub struct CreatedTankEmissionFactorsCalculatedLoader {
    pool: Data<PgPool>,
}

impl CreatedTankEmissionFactorsCalculatedLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedTankEmissionFactorsCalculatedLoader {
    type Value = Vec<TankEmissionFactorCalculated>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_emission_factors_calculated = query_as!(
            TankEmissionFactorCalculated,
            "SELECT * FROM tank_emission_factor_calculated WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_emission_factors_calculated.sort_by_key(|tank_emission_factor_calculated| {
            tank_emission_factor_calculated.created_by_id
        });

        let tank_emission_factors_calculated = tank_emission_factors_calculated
            .into_iter()
            .group_by(|tank_emission_factor_calculated| {
                tank_emission_factor_calculated.created_by_id
            })
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(tank_emission_factors_calculated)
    }
}

pub struct UpdatedTankEmissionFactorsCalculatedLoader {
    pool: Data<PgPool>,
}

impl UpdatedTankEmissionFactorsCalculatedLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedTankEmissionFactorsCalculatedLoader {
    type Value = Vec<TankEmissionFactorCalculated>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_emission_factors_calculated = query_as!(
            TankEmissionFactorCalculated,
            "SELECT * FROM tank_emission_factor_calculated WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_emission_factors_calculated.sort_by_key(|tank_emission_factor_calculated| {
            tank_emission_factor_calculated.updated_by_id
        });

        let tank_emission_factors_calculated = tank_emission_factors_calculated
            .into_iter()
            .group_by(|tank_emission_factor_calculated| {
                tank_emission_factor_calculated.updated_by_id
            })
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(tank_emission_factors_calculated)
    }
}

pub struct TankEmissionFactorsCalculatedByTankLoader {
    pool: Data<PgPool>,
}

impl TankEmissionFactorsCalculatedByTankLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankEmissionFactorsCalculatedByTankLoader {
    type Value = Vec<TankEmissionFactorCalculated>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_emission_factors_calculated = query_as!(
            TankEmissionFactorCalculated,
            "SELECT * FROM tank_emission_factor_calculated WHERE tank_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_emission_factors_calculated
            .sort_by_key(|tank_emission_factor_calculated| tank_emission_factor_calculated.tank_id);

        let tank_emission_factors_calculated = tank_emission_factors_calculated
            .into_iter()
            .group_by(|tank_emission_factor_calculated| tank_emission_factor_calculated.tank_id)
            .into_iter()
            .map(|(tank_id, group)| (tank_id, group.collect()))
            .collect();

        Ok(tank_emission_factors_calculated)
    }
}
