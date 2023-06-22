use crate::graphql::models::routine::pneumatic_device::level_controller::LevelControllerControlledCharacterization;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct LevelControllerControlledCharacterizationLoader {
    pool: Data<PgPool>,
}

impl LevelControllerControlledCharacterizationLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for LevelControllerControlledCharacterizationLoader {
    type Value = LevelControllerControlledCharacterization;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let level_controller_controlled_characterizations = query_as!(
            LevelControllerControlledCharacterization,
            r#"
            SELECT
            id, level_controller_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM level_controller_controlled_characterization
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|level_controller_controlled_characterization| (level_controller_controlled_characterization.id, level_controller_controlled_characterization))
        .collect();

        Ok(level_controller_controlled_characterizations)
    }
}

pub struct CreatedLevelControllerControlledCharacterizationsLoader {
    pool: Data<PgPool>,
}

impl CreatedLevelControllerControlledCharacterizationsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedLevelControllerControlledCharacterizationsLoader {
    type Value = Vec<LevelControllerControlledCharacterization>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_controlled_characterizations = query_as!(
            LevelControllerControlledCharacterization,
            r#"
            SELECT
            id, level_controller_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM level_controller_controlled_characterization
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_controlled_characterizations.sort_by_key(
            |level_controller_controlled_characterization| {
                level_controller_controlled_characterization.created_by_id
            },
        );

        let level_controller_controlled_characterizations =
            level_controller_controlled_characterizations
                .into_iter()
                .group_by(|cf| cf.created_by_id)
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(level_controller_controlled_characterizations)
    }
}

pub struct UpdatedLevelControllerControlledCharacterizationsLoader {
    pool: Data<PgPool>,
}

impl UpdatedLevelControllerControlledCharacterizationsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedLevelControllerControlledCharacterizationsLoader {
    type Value = Vec<LevelControllerControlledCharacterization>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_controlled_characterizations = query_as!(
            LevelControllerControlledCharacterization,
            r#"
            SELECT
            id, level_controller_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM level_controller_controlled_characterization
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_controlled_characterizations.sort_by_key(
            |level_controller_controlled_characterization| {
                level_controller_controlled_characterization.updated_by_id
            },
        );

        let level_controller_controlled_characterizations =
            level_controller_controlled_characterizations
                .into_iter()
                .group_by(|level_controller_controlled_characterization| {
                    level_controller_controlled_characterization.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(level_controller_controlled_characterizations)
    }
}

pub struct LevelControllerControlledCharacterizationsByLevelControllerLoader {
    pool: Data<PgPool>,
}

impl LevelControllerControlledCharacterizationsByLevelControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for LevelControllerControlledCharacterizationsByLevelControllerLoader {
    type Value = Vec<LevelControllerControlledCharacterization>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_controlled_characterizations = sqlx::query_as!(
            LevelControllerControlledCharacterization,
            r#"
            SELECT
            id, level_controller_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM level_controller_controlled_characterization
            WHERE level_controller_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_controlled_characterizations.sort_by_key(
            |level_controller_controlled_characterization| {
                level_controller_controlled_characterization.level_controller_id
            },
        );

        let level_controller_controlled_characterizations =
            level_controller_controlled_characterizations
                .into_iter()
                .group_by(|level_controller_controlled_characterization| {
                    level_controller_controlled_characterization.level_controller_id
                })
                .into_iter()
                .map(|(level_controller_id, group)| (level_controller_id, group.collect()))
                .collect();

        Ok(level_controller_controlled_characterizations)
    }
}
