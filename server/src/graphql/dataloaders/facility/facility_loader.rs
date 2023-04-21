use crate::graphql::models::Facility;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CreatedFacilitiesLoader {
    pool: Data<PgPool>,
}

impl CreatedFacilitiesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedFacilitiesLoader {
    type Value = Vec<Facility>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut facilities = sqlx::query_as!(
            Facility,
            r#"SELECT
            id, idpa, name, type as "type: _", created_by_id, created_at, updated_by_id, updated_at
            FROM facility
            WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        facilities.sort_by_key(|facility| facility.created_by_id);

        let created_facilities = facilities
            .into_iter()
            .group_by(|facility| facility.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_facilities)
    }
}

pub struct UpdatedFacilitiesLoader {
    pool: Data<PgPool>,
}

impl UpdatedFacilitiesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedFacilitiesLoader {
    type Value = Vec<Facility>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut facilities = sqlx::query_as!(
            Facility,
            r#"SELECT
            id, idpa, name, type as "type: _", created_by_id, created_at, updated_by_id, updated_at
            FROM facility
            WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        facilities.sort_by_key(|facility| facility.updated_by_id);

        let updated_facilities = facilities
            .into_iter()
            .group_by(|facility| facility.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_facilities)
    }
}

pub struct FacilityLoader {
    pool: Data<PgPool>,
}

impl FacilityLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for FacilityLoader {
    type Value = Facility;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let facilities = sqlx::query_as!(
            Facility,
            r#"SELECT
            id, idpa, name, type as "type: _", created_by_id, created_at, updated_by_id, updated_at
            FROM facility
            WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|facility| (facility.id, facility))
        .collect();

        Ok(facilities)
    }
}
