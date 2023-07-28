use crate::graphql::models::routine::pneumatic_device::pneumatic_instrument::PneumaticInstrumentEmissionRate;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticInstrumentEmissionRateLoader {
    pool: Data<PgPool>,
}

impl PneumaticInstrumentEmissionRateLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticInstrumentEmissionRateLoader {
    type Value = PneumaticInstrumentEmissionRate;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_instrument_emission_rates = query_as!(
            PneumaticInstrumentEmissionRate,
            "SELECT * FROM pneumatic_instrument_emission_rate WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_instrument_emission_rate| {
            (
                pneumatic_instrument_emission_rate.id,
                pneumatic_instrument_emission_rate,
            )
        })
        .collect();

        Ok(pneumatic_instrument_emission_rates)
    }
}

pub struct CreatedPneumaticInstrumentEmissionRatesLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticInstrumentEmissionRatesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedPneumaticInstrumentEmissionRatesLoader {
    type Value = Vec<PneumaticInstrumentEmissionRate>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_emission_rates = query_as!(
            PneumaticInstrumentEmissionRate,
            "SELECT * FROM pneumatic_instrument_emission_rate WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_emission_rates.sort_by_key(|pneumatic_instrument_emission_rate| {
            pneumatic_instrument_emission_rate.created_by_id
        });

        let pneumatic_instrument_emission_rates = pneumatic_instrument_emission_rates
            .into_iter()
            .group_by(|pneumatic_instrument_emission_rate| {
                pneumatic_instrument_emission_rate.created_by_id
            })
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(pneumatic_instrument_emission_rates)
    }
}

pub struct UpdatedPneumaticInstrumentEmissionRatesLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticInstrumentEmissionRatesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedPneumaticInstrumentEmissionRatesLoader {
    type Value = Vec<PneumaticInstrumentEmissionRate>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_emission_rates = query_as!(
            PneumaticInstrumentEmissionRate,
            "SELECT * FROM pneumatic_instrument_emission_rate WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_emission_rates
            .sort_by_key(|pneumatic_instrument| pneumatic_instrument.updated_by_id);

        let pneumatic_instrument_emission_rates = pneumatic_instrument_emission_rates
            .into_iter()
            .group_by(|pneumatic_instrument| pneumatic_instrument.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(pneumatic_instrument_emission_rates)
    }
}

pub struct PneumaticInstrumentEmissionRatesByPneumaticInstrumentLoader {
    pool: Data<PgPool>,
}

impl PneumaticInstrumentEmissionRatesByPneumaticInstrumentLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticInstrumentEmissionRatesByPneumaticInstrumentLoader {
    type Value = Vec<PneumaticInstrumentEmissionRate>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_emission_rate = query_as!(
            PneumaticInstrumentEmissionRate,
            "SELECT * FROM pneumatic_instrument_emission_rate WHERE pneumatic_instrument_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_emission_rate.sort_by_key(|pneumatic_instrument_emission_rate| {
            pneumatic_instrument_emission_rate.pneumatic_instrument_id
        });

        let pneumatic_instrument_emission_rates = pneumatic_instrument_emission_rate
            .into_iter()
            .group_by(|pneumatic_instrument_emission_rate| {
                pneumatic_instrument_emission_rate.pneumatic_instrument_id
            })
            .into_iter()
            .map(|(pneumatic_instrument_id, group)| (pneumatic_instrument_id, group.collect()))
            .collect();

        Ok(pneumatic_instrument_emission_rates)
    }
}
