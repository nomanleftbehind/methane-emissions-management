use crate::graphql::models::pneumatic_device::non_level_controller::NonLevelControllerMonthHours;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct NonLevelControllerMonthHoursLoader {
    pool: Data<PgPool>,
}

impl NonLevelControllerMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for NonLevelControllerMonthHoursLoader {
    type Value = NonLevelControllerMonthHours;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let non_level_controller_month_hours = query_as!(
            NonLevelControllerMonthHours,
            "SELECT * FROM non_level_controller_month_hours WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|non_level_controller_month_hours| {
            (
                non_level_controller_month_hours.id,
                non_level_controller_month_hours,
            )
        })
        .collect();

        Ok(non_level_controller_month_hours)
    }
}

pub struct CreatedNonLevelControllerMonthHoursLoader {
    pool: Data<PgPool>,
}

impl CreatedNonLevelControllerMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedNonLevelControllerMonthHoursLoader {
    type Value = Vec<NonLevelControllerMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut non_level_controller_month_hours = query_as!(
            NonLevelControllerMonthHours,
            "SELECT * FROM non_level_controller_month_hours WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        non_level_controller_month_hours.sort_by_key(|non_level_controller_month_hours| {
            non_level_controller_month_hours.created_by_id
        });

        let non_level_controller_month_hours = non_level_controller_month_hours
            .into_iter()
            .group_by(|non_level_controller_month_hours| {
                non_level_controller_month_hours.created_by_id
            })
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(non_level_controller_month_hours)
    }
}

pub struct UpdatedNonLevelControllerMonthHoursLoader {
    pool: Data<PgPool>,
}

impl UpdatedNonLevelControllerMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedNonLevelControllerMonthHoursLoader {
    type Value = Vec<NonLevelControllerMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut non_level_controller_month_hours = query_as!(
            NonLevelControllerMonthHours,
            "SELECT * FROM non_level_controller_month_hours WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        non_level_controller_month_hours.sort_by_key(|non_level_controller_month_hours| {
            non_level_controller_month_hours.updated_by_id
        });

        let non_level_controller_month_hours = non_level_controller_month_hours
            .into_iter()
            .group_by(|non_level_controller_month_hours| {
                non_level_controller_month_hours.updated_by_id
            })
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(non_level_controller_month_hours)
    }
}

pub struct NonLevelControllerMonthHoursByNonLevelControllerLoader {
    pool: Data<PgPool>,
}

impl NonLevelControllerMonthHoursByNonLevelControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for NonLevelControllerMonthHoursByNonLevelControllerLoader {
    type Value = Vec<NonLevelControllerMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut non_level_controller_month_hours = query_as!(
            NonLevelControllerMonthHours,
            "SELECT * FROM non_level_controller_month_hours WHERE non_level_controller_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        non_level_controller_month_hours.sort_by_key(|non_level_controller_month_hours| {
            non_level_controller_month_hours.non_level_controller_id
        });

        let non_level_controller_month_hours = non_level_controller_month_hours
            .into_iter()
            .group_by(|non_level_controller_month_hours| {
                non_level_controller_month_hours.non_level_controller_id
            })
            .into_iter()
            .map(|(non_level_controller_id, group)| (non_level_controller_id, group.collect()))
            .collect();

        Ok(non_level_controller_month_hours)
    }
}
