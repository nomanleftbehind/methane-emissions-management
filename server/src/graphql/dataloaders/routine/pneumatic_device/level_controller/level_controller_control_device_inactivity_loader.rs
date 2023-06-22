use crate::graphql::models::routine::pneumatic_device::level_controller::LevelControllerControlDeviceInactivity;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct LevelControllerControlDeviceInactivityLoader {
    pool: Data<PgPool>,
}

impl LevelControllerControlDeviceInactivityLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for LevelControllerControlDeviceInactivityLoader {
    type Value = LevelControllerControlDeviceInactivity;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let level_controller_control_device_inactivities = query_as!(
            LevelControllerControlDeviceInactivity,
            r#"
            SELECT
            id, level_controller_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM level_controller_control_device_inactivity
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|level_controller_control_device_inactivity| (level_controller_control_device_inactivity.id, level_controller_control_device_inactivity))
        .collect();

        Ok(level_controller_control_device_inactivities)
    }
}

pub struct CreatedLevelControllerControlDeviceInactivitiesLoader {
    pool: Data<PgPool>,
}

impl CreatedLevelControllerControlDeviceInactivitiesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedLevelControllerControlDeviceInactivitiesLoader {
    type Value = Vec<LevelControllerControlDeviceInactivity>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_control_device_inactivities = query_as!(
            LevelControllerControlDeviceInactivity,
            r#"
            SELECT
            id, level_controller_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM level_controller_control_device_inactivity
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_control_device_inactivities.sort_by_key(
            |level_controller_control_device_inactivity| {
                level_controller_control_device_inactivity.created_by_id
            },
        );

        let level_controller_control_device_inactivities =
            level_controller_control_device_inactivities
                .into_iter()
                .group_by(|level_controller_control_device_inactivity| {
                    level_controller_control_device_inactivity.created_by_id
                })
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(level_controller_control_device_inactivities)
    }
}

pub struct UpdatedLevelControllerControlDeviceInactivitiesLoader {
    pool: Data<PgPool>,
}

impl UpdatedLevelControllerControlDeviceInactivitiesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedLevelControllerControlDeviceInactivitiesLoader {
    type Value = Vec<LevelControllerControlDeviceInactivity>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_control_device_inactivities = query_as!(
            LevelControllerControlDeviceInactivity,
            r#"
            SELECT
            id, level_controller_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM level_controller_control_device_inactivity
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_control_device_inactivities.sort_by_key(
            |level_controller_control_device_inactivity| {
                level_controller_control_device_inactivity.updated_by_id
            },
        );

        let level_controller_control_device_inactivities =
            level_controller_control_device_inactivities
                .into_iter()
                .group_by(|level_controller_control_device_inactivity| {
                    level_controller_control_device_inactivity.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(level_controller_control_device_inactivities)
    }
}

pub struct LevelControllerControlDeviceInactivitiesByLevelControllerControlledCharacterizationLoader
{
    pool: Data<PgPool>,
}

impl LevelControllerControlDeviceInactivitiesByLevelControllerControlledCharacterizationLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid>
    for LevelControllerControlDeviceInactivitiesByLevelControllerControlledCharacterizationLoader
{
    type Value = Vec<LevelControllerControlDeviceInactivity>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_control_device_inactivities = query_as!(
            LevelControllerControlDeviceInactivity,
            r#"
            SELECT
            id, level_controller_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM level_controller_control_device_inactivity
            WHERE level_controller_controlled_characterization_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_control_device_inactivities.sort_by_key(
            |level_controller_control_device_inactivity| {
                level_controller_control_device_inactivity
                    .level_controller_controlled_characterization_id
            },
        );

        let level_controller_control_device_inactivities =
            level_controller_control_device_inactivities
                .into_iter()
                .group_by(|level_controller_control_device_inactivity| {
                    level_controller_control_device_inactivity
                        .level_controller_controlled_characterization_id
                })
                .into_iter()
                .map(|(level_controller_controlled_characterization_id, group)| {
                    (
                        level_controller_controlled_characterization_id,
                        group.collect(),
                    )
                })
                .collect();

        Ok(level_controller_control_device_inactivities)
    }
}
