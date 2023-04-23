use crate::graphql::models::gas_analysis::GasAnalysis;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct GasAnalysisLoader {
    pool: Data<PgPool>,
}

impl GasAnalysisLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for GasAnalysisLoader {
    type Value = GasAnalysis;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let gas_analyses = query_as!(
            GasAnalysis,
            "SELECT * FROM gas_analysis WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|gas_analysis| (gas_analysis.id, gas_analysis))
        .collect();

        Ok(gas_analyses)
    }
}

pub struct CreatedGasAnalysesLoader {
    pool: Data<PgPool>,
}

impl CreatedGasAnalysesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedGasAnalysesLoader {
    type Value = Vec<GasAnalysis>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut gas_analyses = query_as!(
            GasAnalysis,
            "SELECT * FROM gas_analysis WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        gas_analyses.sort_by_key(|gas_analysis| gas_analysis.created_by_id);

        let gas_analyses = gas_analyses
            .into_iter()
            .group_by(|gas_analysis| gas_analysis.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(gas_analyses)
    }
}

pub struct UpdatedGasAnalysesLoader {
    pool: Data<PgPool>,
}

impl UpdatedGasAnalysesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedGasAnalysesLoader {
    type Value = Vec<GasAnalysis>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut gas_analyses = query_as!(
            GasAnalysis,
            "SELECT * FROM gas_analysis WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        gas_analyses.sort_by_key(|gas_analysis| gas_analysis.updated_by_id);

        let gas_analyses = gas_analyses
            .into_iter()
            .group_by(|gas_analysis| gas_analysis.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(gas_analyses)
    }
}

pub struct GasAnalysesByFacilityLoader {
    pool: Data<PgPool>,
}

impl GasAnalysesByFacilityLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for GasAnalysesByFacilityLoader {
    type Value = Vec<GasAnalysis>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut gas_analyses = query_as!(
            GasAnalysis,
            "SELECT * FROM gas_analysis WHERE facility_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        gas_analyses.sort_by_key(|gas_analysis| gas_analysis.facility_id);

        let gas_analyses = gas_analyses
            .into_iter()
            .group_by(|gas_analysis| gas_analysis.facility_id)
            .into_iter()
            .map(|(facility_id, group)| (facility_id, group.collect()))
            .collect();

        Ok(gas_analyses)
    }
}
