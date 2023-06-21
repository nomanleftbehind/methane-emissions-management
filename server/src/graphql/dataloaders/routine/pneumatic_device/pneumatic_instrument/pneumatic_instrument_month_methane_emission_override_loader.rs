use crate::graphql::models::routine::pneumatic_device::pneumatic_instrument::PneumaticInstrumentMonthMethaneEmissionOverride;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticInstrumentMonthMethaneEmissionOverrideLoader {
    pool: Data<PgPool>,
}

impl PneumaticInstrumentMonthMethaneEmissionOverrideLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticInstrumentMonthMethaneEmissionOverrideLoader {
    type Value = PneumaticInstrumentMonthMethaneEmissionOverride;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_instrument_month_methane_emission_overrides = query_as!(
            PneumaticInstrumentMonthMethaneEmissionOverride,
            "SELECT * FROM pneumatic_instrument_month_methane_emission_override WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_instrument_month_methane_emission_override| {
            (
                pneumatic_instrument_month_methane_emission_override.id,
                pneumatic_instrument_month_methane_emission_override,
            )
        })
        .collect();

        Ok(pneumatic_instrument_month_methane_emission_overrides)
    }
}

pub struct CreatedPneumaticInstrumentMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticInstrumentMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedPneumaticInstrumentMonthMethaneEmissionOverridesLoader {
    type Value = Vec<PneumaticInstrumentMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_month_methane_emission_overrides = query_as!(
            PneumaticInstrumentMonthMethaneEmissionOverride,
            "SELECT * FROM pneumatic_instrument_month_methane_emission_override WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_month_methane_emission_overrides.sort_by_key(
            |pneumatic_instrument_month_methane_emission_override| {
                pneumatic_instrument_month_methane_emission_override.created_by_id
            },
        );

        let created_pneumatic_instrument_month_methane_emission_overrides =
            pneumatic_instrument_month_methane_emission_overrides
                .into_iter()
                .group_by(|pneumatic_instrument_month_methane_emission_override| {
                    pneumatic_instrument_month_methane_emission_override.created_by_id
                })
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(created_pneumatic_instrument_month_methane_emission_overrides)
    }
}

pub struct UpdatedPneumaticInstrumentMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticInstrumentMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedPneumaticInstrumentMonthMethaneEmissionOverridesLoader {
    type Value = Vec<PneumaticInstrumentMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_month_methane_emission_overrides = query_as!(
            PneumaticInstrumentMonthMethaneEmissionOverride,
            "SELECT * FROM pneumatic_instrument_month_methane_emission_override WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_month_methane_emission_overrides.sort_by_key(
            |pneumatic_instrument_month_methane_emission_override| {
                pneumatic_instrument_month_methane_emission_override.updated_by_id
            },
        );

        let updated_pneumatic_instrument_month_methane_emission_overrides =
            pneumatic_instrument_month_methane_emission_overrides
                .into_iter()
                .group_by(|pneumatic_instrument_month_methane_emission_override| {
                    pneumatic_instrument_month_methane_emission_override.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(updated_pneumatic_instrument_month_methane_emission_overrides)
    }
}

pub struct PneumaticInstrumentMonthMethaneEmissionOverridesByPneumaticInstrumentLoader {
    pool: Data<PgPool>,
}

impl PneumaticInstrumentMonthMethaneEmissionOverridesByPneumaticInstrumentLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticInstrumentMonthMethaneEmissionOverridesByPneumaticInstrumentLoader {
    type Value = Vec<PneumaticInstrumentMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_month_methane_emission_overrides = query_as!(
            PneumaticInstrumentMonthMethaneEmissionOverride,
            "SELECT * FROM pneumatic_instrument_month_methane_emission_override WHERE pneumatic_instrument_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_month_methane_emission_overrides.sort_by_key(
            |pneumatic_instrument_month_methane_emission_override| {
                pneumatic_instrument_month_methane_emission_override.pneumatic_instrument_id
            },
        );

        let pneumatic_instrument_month_methane_emission_overrides =
            pneumatic_instrument_month_methane_emission_overrides
                .into_iter()
                .group_by(|pneumatic_instrument_month_methane_emission_override| {
                    pneumatic_instrument_month_methane_emission_override.pneumatic_instrument_id
                })
                .into_iter()
                .map(|(pneumatic_instrument_id, group)| (pneumatic_instrument_id, group.collect()))
                .collect();

        Ok(pneumatic_instrument_month_methane_emission_overrides)
    }
}
