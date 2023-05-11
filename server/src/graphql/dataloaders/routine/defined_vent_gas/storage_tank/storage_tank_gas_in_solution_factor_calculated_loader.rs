use crate::graphql::models::routine::defined_vent_gas::storage_tank::StorageTankGasInSolutionFactorCalculated;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct StorageTankGasInSolutionFactorCalculatedLoader {
    pool: Data<PgPool>,
}

impl StorageTankGasInSolutionFactorCalculatedLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for StorageTankGasInSolutionFactorCalculatedLoader {
    type Value = StorageTankGasInSolutionFactorCalculated;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let storage_tank_gas_in_solution_factor_calculated = query_as!(
            StorageTankGasInSolutionFactorCalculated,
            "SELECT * FROM storage_tank_gas_in_solution_factor_calculated WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|storage_tank_gas_in_solution_factor_calculated| {
            (
                storage_tank_gas_in_solution_factor_calculated.id,
                storage_tank_gas_in_solution_factor_calculated,
            )
        })
        .collect();

        Ok(storage_tank_gas_in_solution_factor_calculated)
    }
}

pub struct CreatedStorageTankGasInSolutionFactorsCalculatedLoader {
    pool: Data<PgPool>,
}

impl CreatedStorageTankGasInSolutionFactorsCalculatedLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedStorageTankGasInSolutionFactorsCalculatedLoader {
    type Value = Vec<StorageTankGasInSolutionFactorCalculated>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_gas_in_solution_factors_calculated = query_as!(
            StorageTankGasInSolutionFactorCalculated,
            "SELECT * FROM storage_tank_gas_in_solution_factor_calculated WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_gas_in_solution_factors_calculated.sort_by_key(
            |storage_tank_gas_in_solution_factor_calculated| {
                storage_tank_gas_in_solution_factor_calculated.created_by_id
            },
        );

        let storage_tank_gas_in_solution_factors_calculated =
            storage_tank_gas_in_solution_factors_calculated
                .into_iter()
                .group_by(|storage_tank_gas_in_solution_factor_calculated| {
                    storage_tank_gas_in_solution_factor_calculated.created_by_id
                })
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(storage_tank_gas_in_solution_factors_calculated)
    }
}

pub struct UpdatedStorageTankGasInSolutionFactorsCalculatedLoader {
    pool: Data<PgPool>,
}

impl UpdatedStorageTankGasInSolutionFactorsCalculatedLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedStorageTankGasInSolutionFactorsCalculatedLoader {
    type Value = Vec<StorageTankGasInSolutionFactorCalculated>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_gas_in_solution_factors_calculated = query_as!(
            StorageTankGasInSolutionFactorCalculated,
            "SELECT * FROM storage_tank_gas_in_solution_factor_calculated WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_gas_in_solution_factors_calculated.sort_by_key(
            |storage_tank_gas_in_solution_factor_calculated| {
                storage_tank_gas_in_solution_factor_calculated.updated_by_id
            },
        );

        let storage_tank_gas_in_solution_factors_calculated =
            storage_tank_gas_in_solution_factors_calculated
                .into_iter()
                .group_by(|storage_tank_gas_in_solution_factor_calculated| {
                    storage_tank_gas_in_solution_factor_calculated.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(storage_tank_gas_in_solution_factors_calculated)
    }
}

pub struct StorageTankGasInSolutionFactorsCalculatedByStorageTankLoader {
    pool: Data<PgPool>,
}

impl StorageTankGasInSolutionFactorsCalculatedByStorageTankLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for StorageTankGasInSolutionFactorsCalculatedByStorageTankLoader {
    type Value = Vec<StorageTankGasInSolutionFactorCalculated>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_gas_in_solution_factors_calculated = query_as!(
            StorageTankGasInSolutionFactorCalculated,
            "SELECT * FROM storage_tank_gas_in_solution_factor_calculated WHERE storage_tank_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_gas_in_solution_factors_calculated.sort_by_key(
            |storage_tank_gas_in_solution_factor_calculated| {
                storage_tank_gas_in_solution_factor_calculated.storage_tank_id
            },
        );

        let storage_tank_gas_in_solution_factors_calculated =
            storage_tank_gas_in_solution_factors_calculated
                .into_iter()
                .group_by(|storage_tank_gas_in_solution_factor_calculated| {
                    storage_tank_gas_in_solution_factor_calculated.storage_tank_id
                })
                .into_iter()
                .map(|(storage_tank_id, group)| (storage_tank_id, group.collect()))
                .collect();

        Ok(storage_tank_gas_in_solution_factors_calculated)
    }
}
