use crate::graphql::models::routine::pneumatic_device::pneumatic_pump::PneumaticPumpEmissionRate;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticPumpEmissionRateLoader {
    pool: Data<PgPool>,
}

impl PneumaticPumpEmissionRateLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for PneumaticPumpEmissionRateLoader {
    type Value = PneumaticPumpEmissionRate;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_pump_emission_rates = query_as!(
            PneumaticPumpEmissionRate,
            "SELECT * FROM pneumatic_pump_emission_rate WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_pump_emission_rate| {
            (
                pneumatic_pump_emission_rate.id,
                pneumatic_pump_emission_rate,
            )
        })
        .collect();

        Ok(pneumatic_pump_emission_rates)
    }
}

pub struct CreatedPneumaticPumpEmissionRatesLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticPumpEmissionRatesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedPneumaticPumpEmissionRatesLoader {
    type Value = Vec<PneumaticPumpEmissionRate>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_emission_rates = query_as!(
            PneumaticPumpEmissionRate,
            "SELECT * FROM pneumatic_pump_emission_rate WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_emission_rates
            .sort_by_key(|pneumatic_pump_emission_rate| pneumatic_pump_emission_rate.created_by_id);

        let pneumatic_pump_emission_rates = pneumatic_pump_emission_rates
            .into_iter()
            .group_by(|pneumatic_pump_emission_rate| pneumatic_pump_emission_rate.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(pneumatic_pump_emission_rates)
    }
}

pub struct UpdatedPneumaticPumpEmissionRatesLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticPumpEmissionRatesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedPneumaticPumpEmissionRatesLoader {
    type Value = Vec<PneumaticPumpEmissionRate>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_emission_rates = query_as!(
            PneumaticPumpEmissionRate,
            "SELECT * FROM pneumatic_pump_emission_rate WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_emission_rates.sort_by_key(|pneumatic_pump| pneumatic_pump.updated_by_id);

        let pneumatic_pump_emission_rates = pneumatic_pump_emission_rates
            .into_iter()
            .group_by(|pneumatic_pump| pneumatic_pump.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(pneumatic_pump_emission_rates)
    }
}

pub struct PneumaticPumpEmissionRatesByPneumaticPumpLoader {
    pool: Data<PgPool>,
}

impl PneumaticPumpEmissionRatesByPneumaticPumpLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for PneumaticPumpEmissionRatesByPneumaticPumpLoader {
    type Value = Vec<PneumaticPumpEmissionRate>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_emission_rate = query_as!(
            PneumaticPumpEmissionRate,
            "SELECT * FROM pneumatic_pump_emission_rate WHERE pneumatic_pump_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_emission_rate.sort_by_key(|pneumatic_pump_emission_rate| {
            pneumatic_pump_emission_rate.pneumatic_pump_id
        });

        let pneumatic_pump_emission_rates = pneumatic_pump_emission_rate
            .into_iter()
            .group_by(|pneumatic_pump_emission_rate| pneumatic_pump_emission_rate.pneumatic_pump_id)
            .into_iter()
            .map(|(pneumatic_pump_id, group)| (pneumatic_pump_id, group.collect()))
            .collect();

        Ok(pneumatic_pump_emission_rates)
    }
}
