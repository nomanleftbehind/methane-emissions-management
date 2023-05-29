use crate::graphql::models::routine::defined_vent_gas::storage_tank::StorageTankEmissionSurvey;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct StorageTankEmissionSurveyLoader {
    pool: Data<PgPool>,
}

impl StorageTankEmissionSurveyLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for StorageTankEmissionSurveyLoader {
    type Value = StorageTankEmissionSurvey;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let storage_tank_emission_surveys = query_as!(
            StorageTankEmissionSurvey,
            "SELECT * FROM storage_tank_emission_survey WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|storage_tank_emission_survey| {
            (
                storage_tank_emission_survey.id,
                storage_tank_emission_survey,
            )
        })
        .collect();

        Ok(storage_tank_emission_surveys)
    }
}

pub struct CreatedStorageTankEmissionSurveysLoader {
    pool: Data<PgPool>,
}

impl CreatedStorageTankEmissionSurveysLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedStorageTankEmissionSurveysLoader {
    type Value = Vec<StorageTankEmissionSurvey>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_emission_surveys = query_as!(
            StorageTankEmissionSurvey,
            "SELECT * FROM storage_tank_emission_survey WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_emission_surveys
            .sort_by_key(|storage_tank_emission_survey| storage_tank_emission_survey.created_by_id);

        let storage_tank_emission_surveys = storage_tank_emission_surveys
            .into_iter()
            .group_by(|storage_tank_emission_survey| storage_tank_emission_survey.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(storage_tank_emission_surveys)
    }
}

pub struct UpdatedStorageTankEmissionSurveysLoader {
    pool: Data<PgPool>,
}

impl UpdatedStorageTankEmissionSurveysLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedStorageTankEmissionSurveysLoader {
    type Value = Vec<StorageTankEmissionSurvey>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_emission_surveys = query_as!(
            StorageTankEmissionSurvey,
            "SELECT * FROM storage_tank_emission_survey WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_emission_surveys
            .sort_by_key(|storage_tank_emission_survey| storage_tank_emission_survey.updated_by_id);

        let storage_tank_emission_surveys = storage_tank_emission_surveys
            .into_iter()
            .group_by(|storage_tank_emission_survey| storage_tank_emission_survey.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(storage_tank_emission_surveys)
    }
}

pub struct StorageTankEmissionSurveysByStorageTankLoader {
    pool: Data<PgPool>,
}

impl StorageTankEmissionSurveysByStorageTankLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for StorageTankEmissionSurveysByStorageTankLoader {
    type Value = Vec<StorageTankEmissionSurvey>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_emission_surveys = query_as!(
            StorageTankEmissionSurvey,
            "SELECT * FROM storage_tank_emission_survey WHERE storage_tank_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_emission_surveys.sort_by_key(|storage_tank_emission_survey| {
            storage_tank_emission_survey.storage_tank_id
        });

        let storage_tank_emission_surveys = storage_tank_emission_surveys
            .into_iter()
            .group_by(|storage_tank_emission_survey| storage_tank_emission_survey.storage_tank_id)
            .into_iter()
            .map(|(storage_tank_id, group)| (storage_tank_id, group.collect()))
            .collect();

        Ok(storage_tank_emission_surveys)
    }
}

pub struct StorageTankEmissionSurveysBySurveyEquipmentLoader {
    pool: Data<PgPool>,
}

impl StorageTankEmissionSurveysBySurveyEquipmentLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for StorageTankEmissionSurveysBySurveyEquipmentLoader {
    type Value = Vec<StorageTankEmissionSurvey>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_emission_surveys = query_as!(
            StorageTankEmissionSurvey,
            "SELECT * FROM storage_tank_emission_survey WHERE survey_equipment_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_emission_surveys.sort_by_key(|storage_tank_emission_survey| {
            storage_tank_emission_survey.survey_equipment_id
        });

        let storage_tank_emission_surveys = storage_tank_emission_surveys
            .into_iter()
            .group_by(|storage_tank_emission_survey| {
                storage_tank_emission_survey.survey_equipment_id
            })
            .into_iter()
            .map(|(survey_equipment_id, group)| (survey_equipment_id, group.collect()))
            .collect();

        Ok(storage_tank_emission_surveys)
    }
}
