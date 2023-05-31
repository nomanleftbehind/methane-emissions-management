use crate::graphql::models::routine::compressor_seal::CompressorEmissionSurvey;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct CompressorEmissionSurveyLoader {
    pool: Data<PgPool>,
}

impl CompressorEmissionSurveyLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorEmissionSurveyLoader {
    type Value = CompressorEmissionSurvey;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let compressor_emission_surveys = query_as!(
            CompressorEmissionSurvey,
            "SELECT * FROM compressor_emission_survey WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|compressor_emission_survey| {
            (compressor_emission_survey.id, compressor_emission_survey)
        })
        .collect();

        Ok(compressor_emission_surveys)
    }
}

pub struct CreatedCompressorEmissionSurveysLoader {
    pool: Data<PgPool>,
}

impl CreatedCompressorEmissionSurveysLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedCompressorEmissionSurveysLoader {
    type Value = Vec<CompressorEmissionSurvey>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_emission_surveys = query_as!(
            CompressorEmissionSurvey,
            "SELECT * FROM compressor_emission_survey WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_emission_surveys
            .sort_by_key(|compressor_emission_survey| compressor_emission_survey.created_by_id);

        let compressor_emission_surveys = compressor_emission_surveys
            .into_iter()
            .group_by(|compressor_emission_survey| compressor_emission_survey.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(compressor_emission_surveys)
    }
}

pub struct UpdatedCompressorEmissionSurveysLoader {
    pool: Data<PgPool>,
}

impl UpdatedCompressorEmissionSurveysLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedCompressorEmissionSurveysLoader {
    type Value = Vec<CompressorEmissionSurvey>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_emission_surveys = query_as!(
            CompressorEmissionSurvey,
            "SELECT * FROM compressor_emission_survey WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_emission_surveys
            .sort_by_key(|compressor_emission_survey| compressor_emission_survey.updated_by_id);

        let compressor_emission_surveys = compressor_emission_surveys
            .into_iter()
            .group_by(|compressor_emission_survey| compressor_emission_survey.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(compressor_emission_surveys)
    }
}

pub struct CompressorEmissionSurveysByCompressorLoader {
    pool: Data<PgPool>,
}

impl CompressorEmissionSurveysByCompressorLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorEmissionSurveysByCompressorLoader {
    type Value = Vec<CompressorEmissionSurvey>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_emission_surveys = query_as!(
            CompressorEmissionSurvey,
            "SELECT * FROM compressor_emission_survey WHERE compressor_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_emission_surveys
            .sort_by_key(|compressor_emission_survey| compressor_emission_survey.compressor_id);

        let compressor_emission_surveys = compressor_emission_surveys
            .into_iter()
            .group_by(|compressor_emission_survey| compressor_emission_survey.compressor_id)
            .into_iter()
            .map(|(compressor_id, group)| (compressor_id, group.collect()))
            .collect();

        Ok(compressor_emission_surveys)
    }
}

pub struct CompressorEmissionSurveysBySurveyEquipmentLoader {
    pool: Data<PgPool>,
}

impl CompressorEmissionSurveysBySurveyEquipmentLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorEmissionSurveysBySurveyEquipmentLoader {
    type Value = Vec<CompressorEmissionSurvey>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_emission_surveys = query_as!(
            CompressorEmissionSurvey,
            "SELECT * FROM compressor_emission_survey WHERE survey_equipment_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_emission_surveys.sort_by_key(|compressor_emission_survey| {
            compressor_emission_survey.survey_equipment_id
        });

        let compressor_emission_surveys = compressor_emission_surveys
            .into_iter()
            .group_by(|compressor_emission_survey| compressor_emission_survey.survey_equipment_id)
            .into_iter()
            .map(|(survey_equipment_id, group)| (survey_equipment_id, group.collect()))
            .collect();

        Ok(compressor_emission_surveys)
    }
}
