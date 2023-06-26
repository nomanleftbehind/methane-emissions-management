use crate::graphql::models::routine::pneumatic_device::pneumatic_pump::PneumaticPumpMonthHours;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticPumpMonthHoursLoader {
    pool: Data<PgPool>,
}

impl PneumaticPumpMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticPumpMonthHoursLoader {
    type Value = PneumaticPumpMonthHours;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_pump_month_hours = query_as!(
            PneumaticPumpMonthHours,
            "SELECT * FROM pneumatic_pump_month_hours WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_pump_month_hours| {
            (pneumatic_pump_month_hours.id, pneumatic_pump_month_hours)
        })
        .collect();

        Ok(pneumatic_pump_month_hours)
    }
}

pub struct CreatedPneumaticPumpMonthHoursLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticPumpMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedPneumaticPumpMonthHoursLoader {
    type Value = Vec<PneumaticPumpMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_month_hours = query_as!(
            PneumaticPumpMonthHours,
            "SELECT * FROM pneumatic_pump_month_hours WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_month_hours
            .sort_by_key(|pneumatic_pump_month_hours| pneumatic_pump_month_hours.created_by_id);

        let pneumatic_pump_month_hours = pneumatic_pump_month_hours
            .into_iter()
            .group_by(|pneumatic_pump_month_hours| pneumatic_pump_month_hours.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(pneumatic_pump_month_hours)
    }
}

pub struct UpdatedPneumaticPumpMonthHoursLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticPumpMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedPneumaticPumpMonthHoursLoader {
    type Value = Vec<PneumaticPumpMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_month_hours = query_as!(
            PneumaticPumpMonthHours,
            "SELECT * FROM pneumatic_pump_month_hours WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_month_hours
            .sort_by_key(|pneumatic_pump_month_hours| pneumatic_pump_month_hours.updated_by_id);

        let pneumatic_pump_month_hours = pneumatic_pump_month_hours
            .into_iter()
            .group_by(|pneumatic_pump_month_hours| pneumatic_pump_month_hours.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(pneumatic_pump_month_hours)
    }
}

pub struct PneumaticPumpMonthHoursByPneumaticPumpLoader {
    pool: Data<PgPool>,
}

impl PneumaticPumpMonthHoursByPneumaticPumpLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticPumpMonthHoursByPneumaticPumpLoader {
    type Value = Vec<PneumaticPumpMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_month_hours = query_as!(
            PneumaticPumpMonthHours,
            "SELECT * FROM pneumatic_pump_month_hours WHERE pneumatic_pump_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_month_hours
            .sort_by_key(|pneumatic_pump_month_hours| pneumatic_pump_month_hours.pneumatic_pump_id);

        let pneumatic_pump_month_hours = pneumatic_pump_month_hours
            .into_iter()
            .group_by(|pneumatic_pump_month_hours| pneumatic_pump_month_hours.pneumatic_pump_id)
            .into_iter()
            .map(|(pneumatic_pump_id, group)| (pneumatic_pump_id, group.collect()))
            .collect();

        Ok(pneumatic_pump_month_hours)
    }
}
