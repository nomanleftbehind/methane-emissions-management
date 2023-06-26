use crate::graphql::models::routine::pneumatic_device::pneumatic_pump::PneumaticPumpMonthMethaneEmissionOverride;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticPumpMonthMethaneEmissionOverrideLoader {
    pool: Data<PgPool>,
}

impl PneumaticPumpMonthMethaneEmissionOverrideLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticPumpMonthMethaneEmissionOverrideLoader {
    type Value = PneumaticPumpMonthMethaneEmissionOverride;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_pump_month_methane_emission_overrides = query_as!(
            PneumaticPumpMonthMethaneEmissionOverride,
            "SELECT * FROM pneumatic_pump_month_methane_emission_override WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_pump_month_methane_emission_override| {
            (
                pneumatic_pump_month_methane_emission_override.id,
                pneumatic_pump_month_methane_emission_override,
            )
        })
        .collect();

        Ok(pneumatic_pump_month_methane_emission_overrides)
    }
}

pub struct CreatedPneumaticPumpMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticPumpMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedPneumaticPumpMonthMethaneEmissionOverridesLoader {
    type Value = Vec<PneumaticPumpMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_month_methane_emission_overrides = query_as!(
            PneumaticPumpMonthMethaneEmissionOverride,
            "SELECT * FROM pneumatic_pump_month_methane_emission_override WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_month_methane_emission_overrides.sort_by_key(
            |pneumatic_pump_month_methane_emission_override| {
                pneumatic_pump_month_methane_emission_override.created_by_id
            },
        );

        let created_pneumatic_pump_month_methane_emission_overrides =
            pneumatic_pump_month_methane_emission_overrides
                .into_iter()
                .group_by(|pneumatic_pump_month_methane_emission_override| {
                    pneumatic_pump_month_methane_emission_override.created_by_id
                })
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(created_pneumatic_pump_month_methane_emission_overrides)
    }
}

pub struct UpdatedPneumaticPumpMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticPumpMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedPneumaticPumpMonthMethaneEmissionOverridesLoader {
    type Value = Vec<PneumaticPumpMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_month_methane_emission_overrides = query_as!(
            PneumaticPumpMonthMethaneEmissionOverride,
            "SELECT * FROM pneumatic_pump_month_methane_emission_override WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_month_methane_emission_overrides.sort_by_key(
            |pneumatic_pump_month_methane_emission_override| {
                pneumatic_pump_month_methane_emission_override.updated_by_id
            },
        );

        let updated_pneumatic_pump_month_methane_emission_overrides =
            pneumatic_pump_month_methane_emission_overrides
                .into_iter()
                .group_by(|pneumatic_pump_month_methane_emission_override| {
                    pneumatic_pump_month_methane_emission_override.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(updated_pneumatic_pump_month_methane_emission_overrides)
    }
}

pub struct PneumaticPumpMonthMethaneEmissionOverridesByPneumaticPumpLoader {
    pool: Data<PgPool>,
}

impl PneumaticPumpMonthMethaneEmissionOverridesByPneumaticPumpLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticPumpMonthMethaneEmissionOverridesByPneumaticPumpLoader {
    type Value = Vec<PneumaticPumpMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_month_methane_emission_overrides = query_as!(
            PneumaticPumpMonthMethaneEmissionOverride,
            "SELECT * FROM pneumatic_pump_month_methane_emission_override WHERE pneumatic_pump_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_month_methane_emission_overrides.sort_by_key(
            |pneumatic_pump_month_methane_emission_override| {
                pneumatic_pump_month_methane_emission_override.pneumatic_pump_id
            },
        );

        let pneumatic_pump_month_methane_emission_overrides =
            pneumatic_pump_month_methane_emission_overrides
                .into_iter()
                .group_by(|pneumatic_pump_month_methane_emission_override| {
                    pneumatic_pump_month_methane_emission_override.pneumatic_pump_id
                })
                .into_iter()
                .map(|(pneumatic_pump_id, group)| (pneumatic_pump_id, group.collect()))
                .collect();

        Ok(pneumatic_pump_month_methane_emission_overrides)
    }
}
