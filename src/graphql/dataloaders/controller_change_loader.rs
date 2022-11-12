use crate::graphql::domain::ControllerChange;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CreatedControllerChangesLoader {
    pool: Data<PgPool>,
}

impl CreatedControllerChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedControllerChangesLoader {
    type Value = Vec<ControllerChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_changes = sqlx::query_as!(
            ControllerChange,
            "SELECT * FROM controller_changes WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_changes.sort_by_key(|cf| cf.created_by_id);

        let created_controller_changes = controller_changes
            .into_iter()
            .group_by(|cf| cf.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_controller_changes)
    }
}

pub struct UpdatedControllerChangesLoader {
    pool: Data<PgPool>,
}

impl UpdatedControllerChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedControllerChangesLoader {
    type Value = Vec<ControllerChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_changes = sqlx::query_as!(
            ControllerChange,
            "SELECT * FROM controller_changes WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_changes.sort_by_key(|cf| cf.updated_by_id);

        let updated_controller_changes = controller_changes
            .into_iter()
            .group_by(|cf| cf.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_controller_changes)
    }
}

pub struct ControllerChangeLoader {
    pool: Data<PgPool>,
}

impl ControllerChangeLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ControllerChangeLoader {
    type Value = ControllerChange;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let controller_changes = sqlx::query_as!(
            ControllerChange,
            "SELECT * FROM controller_changes WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|controller_change| (controller_change.id, controller_change))
        .collect();

        Ok(controller_changes)
    }
}

pub struct ControllerChangesByControllerLoader {
    pool: Data<PgPool>,
}

impl ControllerChangesByControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ControllerChangesByControllerLoader {
    type Value = Vec<ControllerChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_changes = sqlx::query_as!(
            ControllerChange,
            "SELECT * FROM controller_changes WHERE controller_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_changes.sort_by_key(|controller_change| controller_change.controller_id);

        let controller_changes_by_controller = controller_changes
            .into_iter()
            .group_by(|controller_change| controller_change.controller_id)
            .into_iter()
            .map(|(controller_id, group)| (controller_id, group.collect()))
            .collect();

        Ok(controller_changes_by_controller)
    }
}
