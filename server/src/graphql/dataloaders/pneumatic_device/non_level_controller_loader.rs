use crate::graphql::models::pneumatic_device::NonLevelController;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct NonLevelControllerLoader {
    pool: Data<PgPool>,
}

impl NonLevelControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for NonLevelControllerLoader {
    type Value = NonLevelController;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let non_level_controllers = query_as!(
            NonLevelController,
            r#"
            SELECT
            id, site_id, type as "type: _", manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at
            FROM non_level_controller
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|non_level_controller| (non_level_controller.id, non_level_controller))
        .collect();

        Ok(non_level_controllers)
    }
}

pub struct CreatedNonLevelControllersLoader {
    pool: Data<PgPool>,
}

impl CreatedNonLevelControllersLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedNonLevelControllersLoader {
    type Value = Vec<NonLevelController>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut non_level_controllers = query_as!(
            NonLevelController,
            r#"
            SELECT
            id, site_id, type as "type: _", manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at
            FROM non_level_controller
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        non_level_controllers
            .sort_by_key(|non_level_controller| non_level_controller.created_by_id);

        let created_non_level_controllers = non_level_controllers
            .into_iter()
            .group_by(|non_level_controller| non_level_controller.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_non_level_controllers)
    }
}

pub struct UpdatedNonLevelControllersLoader {
    pool: Data<PgPool>,
}

impl UpdatedNonLevelControllersLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedNonLevelControllersLoader {
    type Value = Vec<NonLevelController>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut non_level_controllers = query_as!(
            NonLevelController,
            r#"
            SELECT
            id, site_id, type as "type: _", manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at
            FROM non_level_controller
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        non_level_controllers
            .sort_by_key(|non_level_controller| non_level_controller.updated_by_id);

        let updated_non_level_controllers = non_level_controllers
            .into_iter()
            .group_by(|non_level_controller| non_level_controller.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_non_level_controllers)
    }
}

pub struct SiteNonLevelControllersLoader {
    pool: Data<PgPool>,
}

impl SiteNonLevelControllersLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for SiteNonLevelControllersLoader {
    type Value = Vec<NonLevelController>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut non_level_controllers = query_as!(
            NonLevelController,
            r#"
            SELECT
            id, site_id, type as "type: _", manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at
            FROM non_level_controller
            WHERE site_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        non_level_controllers.sort_by_key(|non_level_controller| non_level_controller.site_id);

        let non_level_controllers = non_level_controllers
            .into_iter()
            .group_by(|non_level_controller| non_level_controller.site_id)
            .into_iter()
            .map(|(facility_id, group)| (facility_id, group.collect()))
            .collect();

        Ok(non_level_controllers)
    }
}

pub struct NonLevelControllersByManufacturerLoader {
    pool: Data<PgPool>,
}

impl NonLevelControllersByManufacturerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for NonLevelControllersByManufacturerLoader {
    type Value = Vec<NonLevelController>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut non_level_controllers = query_as!(
            NonLevelController,
            r#"
            SELECT
            id, site_id, type as "type: _", manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at
            FROM non_level_controller
            WHERE manufacturer_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        non_level_controllers
            .sort_by_key(|non_level_controller| non_level_controller.manufacturer_id);

        let non_level_controllers = non_level_controllers
            .into_iter()
            .group_by(|non_level_controller| non_level_controller.manufacturer_id)
            .into_iter()
            .map(|(manufacturer_id, group)| (manufacturer_id, group.collect()))
            .collect();

        Ok(non_level_controllers)
    }
}
