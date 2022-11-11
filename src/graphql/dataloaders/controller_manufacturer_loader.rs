use crate::graphql::domain::ControllerManufacturer;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CreatedControllerManufacturersLoader {
    pool: Data<PgPool>,
}

impl CreatedControllerManufacturersLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedControllerManufacturersLoader {
    type Value = Vec<ControllerManufacturer>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_manufacturers = sqlx::query_as!(
            ControllerManufacturer,
            "SELECT * FROM controller_manufacturers WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_manufacturers.sort_by_key(|cf| cf.created_by_id);

        let created_controller_manufacturers = controller_manufacturers
            .into_iter()
            .group_by(|cf| cf.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_controller_manufacturers)
    }
}

pub struct UpdatedControllerManufacturersLoader {
    pool: Data<PgPool>,
}

impl UpdatedControllerManufacturersLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedControllerManufacturersLoader {
    type Value = Vec<ControllerManufacturer>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_manufacturers = sqlx::query_as!(
            ControllerManufacturer,
            "SELECT * FROM controller_manufacturers WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_manufacturers.sort_by_key(|cf| cf.updated_by_id);

        let updated_controller_manufacturers = controller_manufacturers
            .into_iter()
            .group_by(|cf| cf.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_controller_manufacturers)
    }
}

pub struct ControllerManufacturerLoader {
    pool: Data<PgPool>,
}

impl ControllerManufacturerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ControllerManufacturerLoader {
    type Value = ControllerManufacturer;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let controller_manufacturers = sqlx::query_as!(
            ControllerManufacturer,
            "SELECT * FROM controller_manufacturers WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|controller_manufacturer| (controller_manufacturer.id, controller_manufacturer))
        .collect();

        Ok(controller_manufacturers)
    }
}
