use crate::graphql::models::pneumatic_device::ControllerMonthVentOverride;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct ControllerMonthVentOverrideLoader {
    pool: Data<PgPool>,
}

impl ControllerMonthVentOverrideLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ControllerMonthVentOverrideLoader {
    type Value = ControllerMonthVentOverride;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let controller_month_vent_override_overrides = sqlx::query_as!(
            ControllerMonthVentOverride,
            r#"SELECT * FROM controller_month_vent_override WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|controller_month_vent_override| {
            (
                controller_month_vent_override.id,
                controller_month_vent_override,
            )
        })
        .collect();

        Ok(controller_month_vent_override_overrides)
    }
}

pub struct CreatedControllerMonthVentOverridesLoader {
    pool: Data<PgPool>,
}

impl CreatedControllerMonthVentOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedControllerMonthVentOverridesLoader {
    type Value = Vec<ControllerMonthVentOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_month_vent_override_overrides = sqlx::query_as!(
            ControllerMonthVentOverride,
            r#"SELECT * FROM controller_month_vent_override WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_month_vent_override_overrides.sort_by_key(|controller_month_vent_override| {
            controller_month_vent_override.created_by_id
        });

        let created_controller_month_vent_override_overrides =
            controller_month_vent_override_overrides
                .into_iter()
                .group_by(|controller_month_vent_override| {
                    controller_month_vent_override.created_by_id
                })
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(created_controller_month_vent_override_overrides)
    }
}

pub struct UpdatedControllerMonthVentOverridesLoader {
    pool: Data<PgPool>,
}

impl UpdatedControllerMonthVentOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedControllerMonthVentOverridesLoader {
    type Value = Vec<ControllerMonthVentOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_month_vent_override_overrides = sqlx::query_as!(
            ControllerMonthVentOverride,
            r#"SELECT * FROM controller_month_vent_override WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_month_vent_override_overrides.sort_by_key(|controller_month_vent_override| {
            controller_month_vent_override.updated_by_id
        });

        let updated_controller_month_vent_override_overrides =
            controller_month_vent_override_overrides
                .into_iter()
                .group_by(|controller_month_vent_override| {
                    controller_month_vent_override.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(updated_controller_month_vent_override_overrides)
    }
}

pub struct ControllerMonthVentOverridesByControllerLoader {
    pool: Data<PgPool>,
}

impl ControllerMonthVentOverridesByControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ControllerMonthVentOverridesByControllerLoader {
    type Value = Vec<ControllerMonthVentOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut controller_month_vent_override_overrides = sqlx::query_as!(
            ControllerMonthVentOverride,
            r#"SELECT * FROM controller_month_vent_override WHERE controller_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        controller_month_vent_override_overrides.sort_by_key(|controller_month_vent_override| {
            controller_month_vent_override.controller_id
        });

        let controller_month_vent_override_overrides_by_controller =
            controller_month_vent_override_overrides
                .into_iter()
                .group_by(|controller_month_vent_override| {
                    controller_month_vent_override.controller_id
                })
                .into_iter()
                .map(|(controller_id, group)| (controller_id, group.collect()))
                .collect();

        Ok(controller_month_vent_override_overrides_by_controller)
    }
}
