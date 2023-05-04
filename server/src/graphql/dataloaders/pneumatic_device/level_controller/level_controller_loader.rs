use crate::graphql::models::pneumatic_device::level_controller::LevelController;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct LevelControllerLoader {
    pool: Data<PgPool>,
}

impl LevelControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for LevelControllerLoader {
    type Value = LevelController;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let level_controllers = query_as!(
            LevelController,
            "SELECT * FROM level_controller WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|level_controller| (level_controller.id, level_controller))
        .collect();

        Ok(level_controllers)
    }
}

pub struct CreatedLevelControllersLoader {
    pool: Data<PgPool>,
}

impl CreatedLevelControllersLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedLevelControllersLoader {
    type Value = Vec<LevelController>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controllers = query_as!(
            LevelController,
            "SELECT * FROM level_controller WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        level_controllers.sort_by_key(|level_controller| level_controller.created_by_id);

        let created_level_controllers = level_controllers
            .into_iter()
            .group_by(|level_controller| level_controller.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_level_controllers)
    }
}

pub struct UpdatedLevelControllersLoader {
    pool: Data<PgPool>,
}

impl UpdatedLevelControllersLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedLevelControllersLoader {
    type Value = Vec<LevelController>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controllers = query_as!(
            LevelController,
            "SELECT * FROM level_controller WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        level_controllers.sort_by_key(|level_controller| level_controller.updated_by_id);

        let updated_level_controllers = level_controllers
            .into_iter()
            .group_by(|level_controller| level_controller.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_level_controllers)
    }
}

pub struct SiteLevelControllersLoader {
    pool: Data<PgPool>,
}

impl SiteLevelControllersLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for SiteLevelControllersLoader {
    type Value = Vec<LevelController>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controllers = query_as!(
            LevelController,
            "SELECT * FROM level_controller WHERE site_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        level_controllers.sort_by_key(|level_controller| level_controller.site_id);

        let level_controllers = level_controllers
            .into_iter()
            .group_by(|level_controller| level_controller.site_id)
            .into_iter()
            .map(|(facility_id, group)| (facility_id, group.collect()))
            .collect();

        Ok(level_controllers)
    }
}

pub struct LevelControllersByManufacturerLoader {
    pool: Data<PgPool>,
}

impl LevelControllersByManufacturerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for LevelControllersByManufacturerLoader {
    type Value = Vec<LevelController>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controllers = query_as!(
            LevelController,
            "SELECT * FROM level_controller WHERE manufacturer_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        level_controllers.sort_by_key(|level_controller| level_controller.manufacturer_id);

        let level_controllers = level_controllers
            .into_iter()
            .group_by(|level_controller| level_controller.manufacturer_id)
            .into_iter()
            .map(|(manufacturer_id, group)| (manufacturer_id, group.collect()))
            .collect();

        Ok(level_controllers)
    }
}
