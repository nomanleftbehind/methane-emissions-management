use crate::graphql::domain::Controller;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct ControllerLoader {
    pool: Data<PgPool>,
}

impl ControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ControllerLoader {
    type Value = Controller;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let controllers = sqlx::query_as!(
            Controller,
            "SELECT * FROM controllers WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|controller| (controller.id, controller))
        .collect();

        Ok(controllers)
    }
}

pub struct CreatedControllersLoader {
    pool: Data<PgPool>,
}

impl CreatedControllersLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedControllersLoader {
    type Value = Vec<Controller>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controllers = sqlx::query_as!(
            Controller,
            "SELECT * FROM controllers WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        controllers.sort_by_key(|controller| controller.created_by_id);

        let created_controllers = controllers
            .into_iter()
            .group_by(|controller| controller.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_controllers)
    }
}

pub struct UpdatedControllersLoader {
    pool: Data<PgPool>,
}

impl UpdatedControllersLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedControllersLoader {
    type Value = Vec<Controller>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controllers = sqlx::query_as!(
            Controller,
            "SELECT * FROM controllers WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        controllers.sort_by_key(|controller| controller.updated_by_id);

        let updated_controllers = controllers
            .into_iter()
            .group_by(|controller| controller.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_controllers)
    }
}

pub struct FacilityControllersLoader {
    pool: Data<PgPool>,
}

impl FacilityControllersLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for FacilityControllersLoader {
    type Value = Vec<Controller>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controllers = sqlx::query_as!(
            Controller,
            "SELECT * FROM controllers WHERE facility_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        controllers.sort_by_key(|controller| controller.facility_id);

        let facility_controllers = controllers
            .into_iter()
            .group_by(|controller| controller.facility_id)
            .into_iter()
            .map(|(facility_id, group)| (facility_id, group.collect()))
            .collect();

        Ok(facility_controllers)
    }
}

pub struct ControllersByFunctionLoader {
    pool: Data<PgPool>,
}

impl ControllersByFunctionLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ControllersByFunctionLoader {
    type Value = Vec<Controller>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controllers = sqlx::query_as!(
            Controller,
            "SELECT * FROM controllers WHERE function_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        controllers.sort_by_key(|controller| controller.function_id);

        let controller_function_controllers = controllers
            .into_iter()
            // This query will never return None variant of function_id because SQL is returning only Controllers with non-null function_ids
            // It is safe to unwrap it.
            .group_by(|controller| controller.function_id.unwrap())
            .into_iter()
            .map(|(function_id, group)| (function_id, group.collect()))
            .collect();

        Ok(controller_function_controllers)
    }
}

pub struct ControllersByManufacturerLoader {
    pool: Data<PgPool>,
}

impl ControllersByManufacturerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ControllersByManufacturerLoader {
    type Value = Vec<Controller>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controllers = sqlx::query_as!(
            Controller,
            "SELECT * FROM controllers WHERE manufacturer_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        controllers.sort_by_key(|controller| controller.manufacturer_id);

        let controller_manufacturer_controllers = controllers
            .into_iter()
            .group_by(|controller| controller.manufacturer_id)
            .into_iter()
            .map(|(manufacturer_id, group)| (manufacturer_id, group.collect()))
            .collect();

        Ok(controller_manufacturer_controllers)
    }
}
