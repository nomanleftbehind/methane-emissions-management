use crate::graphql::models::ControllerMonthVent;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct ControllerMonthVentLoader {
    pool: Data<PgPool>,
}

impl ControllerMonthVentLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ControllerMonthVentLoader {
    type Value = ControllerMonthVent;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let controller_month_vents = sqlx::query_as!(
            ControllerMonthVent,
            "SELECT * FROM controller_month_vent WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|controller_month_vent| (controller_month_vent.id, controller_month_vent))
        .collect();

        Ok(controller_month_vents)
    }
}

pub struct CreatedControllerMonthVentsLoader {
    pool: Data<PgPool>,
}

impl CreatedControllerMonthVentsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedControllerMonthVentsLoader {
    type Value = Vec<ControllerMonthVent>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_month_vents = sqlx::query_as!(
            ControllerMonthVent,
            "SELECT * FROM controller_month_vent WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_month_vents
            .sort_by_key(|controller_month_vent| controller_month_vent.created_by_id);

        let created_controller_month_vents = controller_month_vents
            .into_iter()
            .group_by(|controller_month_vent| controller_month_vent.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_controller_month_vents)
    }
}

pub struct UpdatedControllerMonthVentsLoader {
    pool: Data<PgPool>,
}

impl UpdatedControllerMonthVentsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedControllerMonthVentsLoader {
    type Value = Vec<ControllerMonthVent>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_month_vents = sqlx::query_as!(
            ControllerMonthVent,
            "SELECT * FROM controller_month_vent WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_month_vents
            .sort_by_key(|controller_month_vent| controller_month_vent.updated_by_id);

        let updated_controller_month_vents = controller_month_vents
            .into_iter()
            .group_by(|controller_month_vent| controller_month_vent.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_controller_month_vents)
    }
}

pub struct ControllerMonthVentsByControllerLoader {
    pool: Data<PgPool>,
}

impl ControllerMonthVentsByControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ControllerMonthVentsByControllerLoader {
    type Value = Vec<ControllerMonthVent>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_month_vents = sqlx::query_as!(
            ControllerMonthVent,
            "SELECT * FROM controller_month_vent WHERE controller_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_month_vents
            .sort_by_key(|controller_month_vent| controller_month_vent.controller_id);

        let controller_month_vents_by_controller = controller_month_vents
            .into_iter()
            .group_by(|controller_month_vent| controller_month_vent.controller_id)
            .into_iter()
            .map(|(controller_id, group)| (controller_id, group.collect()))
            .collect();

        Ok(controller_month_vents_by_controller)
    }
}
