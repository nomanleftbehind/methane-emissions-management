use crate::graphql::models::survey_equipment::SurveyEquipment;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct SurveyEquipmentLoader {
    pool: Data<PgPool>,
}

impl SurveyEquipmentLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for SurveyEquipmentLoader {
    type Value = SurveyEquipment;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let survey_equipment = query_as!(
            SurveyEquipment,
            r#"SELECT * FROM survey_equipment WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|survey_equipment| (survey_equipment.id, survey_equipment))
        .collect();

        Ok(survey_equipment)
    }
}

pub struct CreatedSurveyEquipmentLoader {
    pool: Data<PgPool>,
}

impl CreatedSurveyEquipmentLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedSurveyEquipmentLoader {
    type Value = Vec<SurveyEquipment>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut survey_equipment = query_as!(
            SurveyEquipment,
            "SELECT * FROM survey_equipment WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        survey_equipment.sort_by_key(|survey_equipment| survey_equipment.created_by_id);

        let survey_equipment = survey_equipment
            .into_iter()
            .group_by(|survey_equipment| survey_equipment.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(survey_equipment)
    }
}

pub struct UpdatedSurveyEquipmentLoader {
    pool: Data<PgPool>,
}

impl UpdatedSurveyEquipmentLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedSurveyEquipmentLoader {
    type Value = Vec<SurveyEquipment>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut survey_equipment = query_as!(
            SurveyEquipment,
            "SELECT * FROM survey_equipment WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        survey_equipment.sort_by_key(|survey_equipment| survey_equipment.updated_by_id);

        let survey_equipment = survey_equipment
            .into_iter()
            .group_by(|survey_equipment| survey_equipment.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(survey_equipment)
    }
}
