use crate::graphql::models::pneumatic_device::PneumaticDeviceMonthHours;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticDeviceMonthHoursLoader {
    pool: Data<PgPool>,
}

impl PneumaticDeviceMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticDeviceMonthHoursLoader {
    type Value = PneumaticDeviceMonthHours;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_device_month_hours = query_as!(
            PneumaticDeviceMonthHours,
            "SELECT * FROM pneumatic_device_month_hours WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_device_month_hours| {
            (
                pneumatic_device_month_hours.id,
                pneumatic_device_month_hours,
            )
        })
        .collect();

        Ok(pneumatic_device_month_hours)
    }
}

pub struct CreatedPneumaticDeviceMonthHoursLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticDeviceMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedPneumaticDeviceMonthHoursLoader {
    type Value = Vec<PneumaticDeviceMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_device_month_hours = query_as!(
            PneumaticDeviceMonthHours,
            "SELECT * FROM pneumatic_device_month_hours WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_device_month_hours
            .sort_by_key(|pneumatic_device_month_hours| pneumatic_device_month_hours.created_by_id);

        let pneumatic_device_month_hours = pneumatic_device_month_hours
            .into_iter()
            .group_by(|pneumatic_device_month_hours| pneumatic_device_month_hours.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(pneumatic_device_month_hours)
    }
}

pub struct UpdatedPneumaticDeviceMonthHoursLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticDeviceMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedPneumaticDeviceMonthHoursLoader {
    type Value = Vec<PneumaticDeviceMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_device_month_hours = query_as!(
            PneumaticDeviceMonthHours,
            "SELECT * FROM pneumatic_device_month_hours WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_device_month_hours
            .sort_by_key(|pneumatic_device_month_hours| pneumatic_device_month_hours.updated_by_id);

        let pneumatic_device_month_hours = pneumatic_device_month_hours
            .into_iter()
            .group_by(|pneumatic_device_month_hours| pneumatic_device_month_hours.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(pneumatic_device_month_hours)
    }
}

pub struct PneumaticDeviceMonthHoursByPneumaticDeviceLoader {
    pool: Data<PgPool>,
}

impl PneumaticDeviceMonthHoursByPneumaticDeviceLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticDeviceMonthHoursByPneumaticDeviceLoader {
    type Value = Vec<PneumaticDeviceMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_device_month_hours = query_as!(
            PneumaticDeviceMonthHours,
            "SELECT * FROM pneumatic_device_month_hours WHERE pneumatic_device_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_device_month_hours.sort_by_key(|pneumatic_device_month_hours| {
            pneumatic_device_month_hours.pneumatic_device_id
        });

        let pneumatic_device_month_hours = pneumatic_device_month_hours
            .into_iter()
            .group_by(|pneumatic_device_month_hours| {
                pneumatic_device_month_hours.pneumatic_device_id
            })
            .into_iter()
            .map(|(pneumatic_device_id, group)| (pneumatic_device_id, group.collect()))
            .collect();

        Ok(pneumatic_device_month_hours)
    }
}
