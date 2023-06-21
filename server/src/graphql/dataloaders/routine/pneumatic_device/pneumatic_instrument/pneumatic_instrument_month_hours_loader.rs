use crate::graphql::models::routine::pneumatic_device::pneumatic_instrument::PneumaticInstrumentMonthHours;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticInstrumentMonthHoursLoader {
    pool: Data<PgPool>,
}

impl PneumaticInstrumentMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticInstrumentMonthHoursLoader {
    type Value = PneumaticInstrumentMonthHours;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_instrument_month_hours = query_as!(
            PneumaticInstrumentMonthHours,
            "SELECT * FROM pneumatic_instrument_month_hours WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_instrument_month_hours| {
            (
                pneumatic_instrument_month_hours.id,
                pneumatic_instrument_month_hours,
            )
        })
        .collect();

        Ok(pneumatic_instrument_month_hours)
    }
}

pub struct CreatedPneumaticInstrumentMonthHoursLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticInstrumentMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedPneumaticInstrumentMonthHoursLoader {
    type Value = Vec<PneumaticInstrumentMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_month_hours = query_as!(
            PneumaticInstrumentMonthHours,
            "SELECT * FROM pneumatic_instrument_month_hours WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_month_hours.sort_by_key(|pneumatic_instrument_month_hours| {
            pneumatic_instrument_month_hours.created_by_id
        });

        let pneumatic_instrument_month_hours = pneumatic_instrument_month_hours
            .into_iter()
            .group_by(|pneumatic_instrument_month_hours| {
                pneumatic_instrument_month_hours.created_by_id
            })
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(pneumatic_instrument_month_hours)
    }
}

pub struct UpdatedPneumaticInstrumentMonthHoursLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticInstrumentMonthHoursLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedPneumaticInstrumentMonthHoursLoader {
    type Value = Vec<PneumaticInstrumentMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_month_hours = query_as!(
            PneumaticInstrumentMonthHours,
            "SELECT * FROM pneumatic_instrument_month_hours WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_month_hours.sort_by_key(|pneumatic_instrument_month_hours| {
            pneumatic_instrument_month_hours.updated_by_id
        });

        let pneumatic_instrument_month_hours = pneumatic_instrument_month_hours
            .into_iter()
            .group_by(|pneumatic_instrument_month_hours| {
                pneumatic_instrument_month_hours.updated_by_id
            })
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(pneumatic_instrument_month_hours)
    }
}

pub struct PneumaticInstrumentMonthHoursByPneumaticInstrumentLoader {
    pool: Data<PgPool>,
}

impl PneumaticInstrumentMonthHoursByPneumaticInstrumentLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticInstrumentMonthHoursByPneumaticInstrumentLoader {
    type Value = Vec<PneumaticInstrumentMonthHours>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_month_hours = query_as!(
            PneumaticInstrumentMonthHours,
            "SELECT * FROM pneumatic_instrument_month_hours WHERE pneumatic_instrument_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_month_hours.sort_by_key(|pneumatic_instrument_month_hours| {
            pneumatic_instrument_month_hours.pneumatic_instrument_id
        });

        let pneumatic_instrument_month_hours = pneumatic_instrument_month_hours
            .into_iter()
            .group_by(|pneumatic_instrument_month_hours| {
                pneumatic_instrument_month_hours.pneumatic_instrument_id
            })
            .into_iter()
            .map(|(pneumatic_instrument_id, group)| (pneumatic_instrument_id, group.collect()))
            .collect();

        Ok(pneumatic_instrument_month_hours)
    }
}
