use crate::graphql::models::routine::compressor_seal::CompressorSealTest;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct CompressorSealTestLoader {
    pool: Data<PgPool>,
}

impl CompressorSealTestLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorSealTestLoader {
    type Value = CompressorSealTest;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let compressor_seal_tests = query_as!(
            CompressorSealTest,
            r#"
            SELECT
            id, compressor_seal_id, date, rate, testing_point as "testing_point: _", survey_equipment_id, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_seal_test
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|compressor_seal_test| (compressor_seal_test.id, compressor_seal_test))
        .collect();

        Ok(compressor_seal_tests)
    }
}

pub struct CreatedCompressorSealTestsLoader {
    pool: Data<PgPool>,
}

impl CreatedCompressorSealTestsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedCompressorSealTestsLoader {
    type Value = Vec<CompressorSealTest>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_seal_tests = query_as!(
            CompressorSealTest,
            r#"
            SELECT
            id, compressor_seal_id, date, rate, testing_point as "testing_point: _", survey_equipment_id, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_seal_test
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_seal_tests
            .sort_by_key(|compressor_seal_test| compressor_seal_test.created_by_id);

        let compressor_seal_tests = compressor_seal_tests
            .into_iter()
            .group_by(|compressor_seal_test| compressor_seal_test.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(compressor_seal_tests)
    }
}

pub struct UpdatedCompressorSealTestsLoader {
    pool: Data<PgPool>,
}

impl UpdatedCompressorSealTestsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedCompressorSealTestsLoader {
    type Value = Vec<CompressorSealTest>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_seal_tests = query_as!(
            CompressorSealTest,
            r#"
            SELECT
            id, compressor_seal_id, date, rate, testing_point as "testing_point: _", survey_equipment_id, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_seal_test
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_seal_tests
            .sort_by_key(|compressor_seal_test| compressor_seal_test.updated_by_id);

        let compressor_seal_tests = compressor_seal_tests
            .into_iter()
            .group_by(|compressor_seal_test| compressor_seal_test.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(compressor_seal_tests)
    }
}

pub struct CompressorSealTestsByCompressorSealLoader {
    pool: Data<PgPool>,
}

impl CompressorSealTestsByCompressorSealLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorSealTestsByCompressorSealLoader {
    type Value = Vec<CompressorSealTest>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_seal_tests = query_as!(
            CompressorSealTest,
            r#"
            SELECT
            id, compressor_seal_id, date, rate, testing_point as "testing_point: _", survey_equipment_id, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_seal_test
            WHERE compressor_seal_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_seal_tests
            .sort_by_key(|compressor_seal_test| compressor_seal_test.compressor_seal_id);

        let compressor_seal_tests = compressor_seal_tests
            .into_iter()
            .group_by(|compressor_seal_test| compressor_seal_test.compressor_seal_id)
            .into_iter()
            .map(|(compressor_seal_id, group)| (compressor_seal_id, group.collect()))
            .collect();

        Ok(compressor_seal_tests)
    }
}

pub struct CompressorSealTestsBySurveyEquipmentLoader {
    pool: Data<PgPool>,
}

impl CompressorSealTestsBySurveyEquipmentLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CompressorSealTestsBySurveyEquipmentLoader {
    type Value = Vec<CompressorSealTest>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_seal_tests = query_as!(
            CompressorSealTest,
            r#"
            SELECT
            id, compressor_seal_id, date, rate, testing_point as "testing_point: _", survey_equipment_id, created_by_id, created_at, updated_by_id, updated_at
            FROM compressor_seal_test
            WHERE survey_equipment_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_seal_tests
            .sort_by_key(|compressor_seal_test| compressor_seal_test.survey_equipment_id);

        let compressor_seal_tests = compressor_seal_tests
            .into_iter()
            .group_by(|compressor_seal_test| compressor_seal_test.survey_equipment_id)
            .into_iter()
            .map(|(survey_equipment_id, group)| (survey_equipment_id, group.collect()))
            .collect();

        Ok(compressor_seal_tests)
    }
}
