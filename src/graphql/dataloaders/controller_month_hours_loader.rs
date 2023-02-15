use crate::graphql::models::ControllerMonthHours;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;


pub struct ControllerMonthHoursLoader {
    pool: Data<PgPool>,
}

impl ControllerMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ControllerMonthHoursLoader {
    type Value = ControllerMonthHours;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let controller_month_hours = sqlx::query_as!(
            ControllerMonthHours,
            "SELECT * FROM controller_month_hours WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|controller_month_hours| (controller_month_hours.id, controller_month_hours))
        .collect();

        Ok(controller_month_hours)
    }
}




pub struct CreatedControllerMonthHoursLoader {
    pool: Data<PgPool>,
}

impl CreatedControllerMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedControllerMonthHoursLoader {
    type Value = Vec<ControllerMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_month_hours_s = sqlx::query_as!(
            ControllerMonthHours,
            "SELECT * FROM controller_month_hours WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_month_hours_s
            .sort_by_key(|controller_month_hours| controller_month_hours.created_by_id);

        let created_controller_month_hours = controller_month_hours_s
            .into_iter()
            .group_by(|cf| cf.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_controller_month_hours)
    }
}

pub struct UpdatedControllerMonthHoursLoader {
    pool: Data<PgPool>,
}

impl UpdatedControllerMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedControllerMonthHoursLoader {
    type Value = Vec<ControllerMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_month_hours_s = sqlx::query_as!(
            ControllerMonthHours,
            "SELECT * FROM controller_month_hours WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_month_hours_s
            .sort_by_key(|controller_month_hours| controller_month_hours.updated_by_id);

        let updated_controller_month_hours = controller_month_hours_s
            .into_iter()
            .group_by(|controller_month_hours| controller_month_hours.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_controller_month_hours)
    }
}

pub struct ControllerMonthHoursByControllerLoader {
    pool: Data<PgPool>,
}

impl ControllerMonthHoursByControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ControllerMonthHoursByControllerLoader {
    type Value = Vec<ControllerMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_month_hours_s = sqlx::query_as!(
            ControllerMonthHours,
            "SELECT * FROM controller_month_hours WHERE controller_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_month_hours_s
            .sort_by_key(|controller_month_hours| controller_month_hours.controller_id);

        let controller_month_hours_by_controller = controller_month_hours_s
            .into_iter()
            .group_by(|controller_month_hours| controller_month_hours.controller_id)
            .into_iter()
            .map(|(controller_id, group)| (controller_id, group.collect()))
            .collect();

        Ok(controller_month_hours_by_controller)
    }
}
