use crate::graphql::domain::ControllerFunction;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CreatedControllerFunctionsLoader {
    pool: Data<PgPool>,
}

impl CreatedControllerFunctionsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedControllerFunctionsLoader {
    type Value = Vec<ControllerFunction>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_functions = sqlx::query_as!(
            ControllerFunction,
            "SELECT * FROM controller_functions WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_functions.sort_by_key(|cf| cf.created_by_id);

        let created_controller_functions = controller_functions
            .into_iter()
            .group_by(|cf| cf.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_controller_functions)
    }
}

pub struct UpdatedControllerFunctionsLoader {
    pool: Data<PgPool>,
}

impl UpdatedControllerFunctionsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedControllerFunctionsLoader {
    type Value = Vec<ControllerFunction>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_functions = sqlx::query_as!(
            ControllerFunction,
            "SELECT * FROM controller_functions WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_functions.sort_by_key(|cf| cf.updated_by_id);

        let updated_controller_functions = controller_functions
            .into_iter()
            .group_by(|cf| cf.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_controller_functions)
    }
}

pub struct ControllerFunctionLoader {
    pool: Data<PgPool>,
}

impl ControllerFunctionLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ControllerFunctionLoader {
    type Value = ControllerFunction;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let controller_functions = sqlx::query_as!(
            ControllerFunction,
            "SELECT * FROM controller_functions WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|controller_function| (controller_function.id, controller_function))
        .collect();

        Ok(controller_functions)
    }
}
