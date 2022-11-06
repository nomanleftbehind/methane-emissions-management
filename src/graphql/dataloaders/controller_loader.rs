use crate::graphql::domain::Controller;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

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
