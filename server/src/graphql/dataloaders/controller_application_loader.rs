use crate::graphql::models::pneumatic_device::NonLevelControllerType;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CreatedControllerApplicationsLoader {
    pool: Data<PgPool>,
}

impl CreatedControllerApplicationsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedControllerApplicationsLoader {
    type Value = Vec<NonLevelControllerType>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_applications = sqlx::query_as!(
            NonLevelControllerType,
            "SELECT * FROM controller_applications WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_applications.sort_by_key(|cf| cf.created_by_id);

        let created_controller_applications = controller_applications
            .into_iter()
            .group_by(|cf| cf.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_controller_applications)
    }
}

pub struct UpdatedControllerApplicationsLoader {
    pool: Data<PgPool>,
}

impl UpdatedControllerApplicationsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedControllerApplicationsLoader {
    type Value = Vec<NonLevelControllerType>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_applications = sqlx::query_as!(
            NonLevelControllerType,
            "SELECT * FROM controller_applications WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_applications.sort_by_key(|cf| cf.updated_by_id);

        let updated_controller_applications = controller_applications
            .into_iter()
            .group_by(|cf| cf.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_controller_applications)
    }
}

pub struct ControllerApplicationLoader {
    pool: Data<PgPool>,
}

impl ControllerApplicationLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ControllerApplicationLoader {
    type Value = NonLevelControllerType;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let controller_applications = sqlx::query_as!(
            NonLevelControllerType,
            "SELECT * FROM controller_applications WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|controller_application| (controller_application.id, controller_application))
        .collect();

        Ok(controller_applications)
    }
}
