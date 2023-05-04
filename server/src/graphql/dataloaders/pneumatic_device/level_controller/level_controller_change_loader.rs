use crate::graphql::models::pneumatic_device::level_controller::LevelControllerChange;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct LevelControllerChangeLoader {
    pool: Data<PgPool>,
}

impl LevelControllerChangeLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for LevelControllerChangeLoader {
    type Value = LevelControllerChange;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let level_controller_changes = query_as!(
            LevelControllerChange,
            "SELECT * FROM level_controller_change WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|level_controller_change| (level_controller_change.id, level_controller_change))
        .collect();

        Ok(level_controller_changes)
    }
}

pub struct CreatedLevelControllerChangesLoader {
    pool: Data<PgPool>,
}

impl CreatedLevelControllerChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedLevelControllerChangesLoader {
    type Value = Vec<LevelControllerChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_changes = query_as!(
            LevelControllerChange,
            "SELECT * FROM level_controller_change WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_changes
            .sort_by_key(|level_controller_change| level_controller_change.created_by_id);

        let level_controller_changes = level_controller_changes
            .into_iter()
            .group_by(|level_controller_change| level_controller_change.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(level_controller_changes)
    }
}

pub struct UpdatedLevelControllerChangesLoader {
    pool: Data<PgPool>,
}

impl UpdatedLevelControllerChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedLevelControllerChangesLoader {
    type Value = Vec<LevelControllerChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_changes = query_as!(
            LevelControllerChange,
            "SELECT * FROM level_controller_change WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_changes.sort_by_key(|level_controller| level_controller.updated_by_id);

        let level_controller_changes = level_controller_changes
            .into_iter()
            .group_by(|level_controller| level_controller.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(level_controller_changes)
    }
}

pub struct LevelControllerChangesByLevelControllerLoader {
    pool: Data<PgPool>,
}

impl LevelControllerChangesByLevelControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for LevelControllerChangesByLevelControllerLoader {
    type Value = Vec<LevelControllerChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_change = query_as!(
            LevelControllerChange,
            "SELECT * FROM level_controller_change WHERE level_controller_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_change
            .sort_by_key(|level_controller_change| level_controller_change.level_controller_id);

        let level_controller_changes = level_controller_change
            .into_iter()
            .group_by(|level_controller_change| level_controller_change.level_controller_id)
            .into_iter()
            .map(|(level_controller_id, group)| (level_controller_id, group.collect()))
            .collect();

        Ok(level_controller_changes)
    }
}
