use crate::graphql::models::GasAnalysisCalculatedParam;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct GasAnalysisCalculatedParamLoader {
    pool: Data<PgPool>,
}

impl GasAnalysisCalculatedParamLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for GasAnalysisCalculatedParamLoader {
    type Value = GasAnalysisCalculatedParam;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let gas_analysis_calculated_params = sqlx::query_as!(
            GasAnalysisCalculatedParam,
            "SELECT * FROM gas_analysis_calculated_params WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|gas_analysis_calculated_param| {
            (
                gas_analysis_calculated_param.id,
                gas_analysis_calculated_param,
            )
        })
        .collect();

        Ok(gas_analysis_calculated_params)
    }
}

pub struct CreatedGasAnalysisCalculatedParamsLoader {
    pool: Data<PgPool>,
}

impl CreatedGasAnalysisCalculatedParamsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedGasAnalysisCalculatedParamsLoader {
    type Value = Vec<GasAnalysisCalculatedParam>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut gas_analysis_calculated_params = sqlx::query_as!(
            GasAnalysisCalculatedParam,
            "SELECT * FROM gas_analysis_calculated_params WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        gas_analysis_calculated_params.sort_by_key(|gas_analysis_calculated_param| {
            gas_analysis_calculated_param.created_by_id
        });

        let created_gas_analysis_calculated_params = gas_analysis_calculated_params
            .into_iter()
            .group_by(|gas_analysis_calculated_param| gas_analysis_calculated_param.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_gas_analysis_calculated_params)
    }
}

pub struct UpdatedGasAnalysisCalculatedParamsLoader {
    pool: Data<PgPool>,
}

impl UpdatedGasAnalysisCalculatedParamsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedGasAnalysisCalculatedParamsLoader {
    type Value = Vec<GasAnalysisCalculatedParam>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut gas_analysis_calculated_params = sqlx::query_as!(
            GasAnalysisCalculatedParam,
            "SELECT * FROM gas_analysis_calculated_params WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        gas_analysis_calculated_params.sort_by_key(|gas_analysis_calculated_param| {
            gas_analysis_calculated_param.updated_by_id
        });

        let updated_gas_analysis_calculated_params = gas_analysis_calculated_params
            .into_iter()
            .group_by(|gas_analysis_calculated_param| gas_analysis_calculated_param.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_gas_analysis_calculated_params)
    }
}
