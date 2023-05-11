use crate::graphql::models::routine::pneumatic_device::level_controller::LevelControllerMonthHours;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct LevelControllerMonthHoursLoader {
    pool: Data<PgPool>,
}

impl LevelControllerMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for LevelControllerMonthHoursLoader {
    type Value = LevelControllerMonthHours;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let level_controller_month_hours = query_as!(
            LevelControllerMonthHours,
            "SELECT * FROM level_controller_month_hours WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|level_controller_month_hours| {
            (
                level_controller_month_hours.id,
                level_controller_month_hours,
            )
        })
        .collect();

        Ok(level_controller_month_hours)
    }
}

pub struct CreatedLevelControllerMonthHoursLoader {
    pool: Data<PgPool>,
}

impl CreatedLevelControllerMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedLevelControllerMonthHoursLoader {
    type Value = Vec<LevelControllerMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_month_hours = query_as!(
            LevelControllerMonthHours,
            "SELECT * FROM level_controller_month_hours WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_month_hours
            .sort_by_key(|level_controller_month_hours| level_controller_month_hours.created_by_id);

        let level_controller_month_hours = level_controller_month_hours
            .into_iter()
            .group_by(|level_controller_month_hours| level_controller_month_hours.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(level_controller_month_hours)
    }
}

pub struct UpdatedLevelControllerMonthHoursLoader {
    pool: Data<PgPool>,
}

impl UpdatedLevelControllerMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedLevelControllerMonthHoursLoader {
    type Value = Vec<LevelControllerMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_month_hours = query_as!(
            LevelControllerMonthHours,
            "SELECT * FROM level_controller_month_hours WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_month_hours
            .sort_by_key(|level_controller_month_hours| level_controller_month_hours.updated_by_id);

        let level_controller_month_hours = level_controller_month_hours
            .into_iter()
            .group_by(|level_controller_month_hours| level_controller_month_hours.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(level_controller_month_hours)
    }
}

pub struct LevelControllerMonthHoursByLevelControllerLoader {
    pool: Data<PgPool>,
}

impl LevelControllerMonthHoursByLevelControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for LevelControllerMonthHoursByLevelControllerLoader {
    type Value = Vec<LevelControllerMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut level_controller_month_hours = query_as!(
            LevelControllerMonthHours,
            "SELECT * FROM level_controller_month_hours WHERE level_controller_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        level_controller_month_hours.sort_by_key(|level_controller_month_hours| {
            level_controller_month_hours.level_controller_id
        });

        let level_controller_month_hours = level_controller_month_hours
            .into_iter()
            .group_by(|level_controller_month_hours| {
                level_controller_month_hours.level_controller_id
            })
            .into_iter()
            .map(|(level_controller_id, group)| (level_controller_id, group.collect()))
            .collect();

        Ok(level_controller_month_hours)
    }
}
