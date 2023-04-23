use crate::graphql::models::pneumatic_device::PneumaticDeviceMonthMethaneEmissionOverride;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticDeviceMonthMethaneEmissionOverrideLoader {
    pool: Data<PgPool>,
}

impl PneumaticDeviceMonthMethaneEmissionOverrideLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticDeviceMonthMethaneEmissionOverrideLoader {
    type Value = PneumaticDeviceMonthMethaneEmissionOverride;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_device_month_methane_emission_overrides = query_as!(
            PneumaticDeviceMonthMethaneEmissionOverride,
            r#"SELECT * FROM pneumatic_device_month_methane_emission_override WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_device_month_methane_emission_override| {
            (
                pneumatic_device_month_methane_emission_override.id,
                pneumatic_device_month_methane_emission_override,
            )
        })
        .collect();

        Ok(pneumatic_device_month_methane_emission_overrides)
    }
}

pub struct CreatedPneumaticDeviceMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticDeviceMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedPneumaticDeviceMonthMethaneEmissionOverridesLoader {
    type Value = Vec<PneumaticDeviceMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_device_month_methane_emission_overrides = query_as!(
            PneumaticDeviceMonthMethaneEmissionOverride,
            r#"SELECT * FROM pneumatic_device_month_methane_emission_override WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_device_month_methane_emission_overrides.sort_by_key(
            |pneumatic_device_month_methane_emission_override| {
                pneumatic_device_month_methane_emission_override.created_by_id
            },
        );

        let created_pneumatic_device_month_methane_emission_overrides =
            pneumatic_device_month_methane_emission_overrides
                .into_iter()
                .group_by(|pneumatic_device_month_methane_emission_override| {
                    pneumatic_device_month_methane_emission_override.created_by_id
                })
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(created_pneumatic_device_month_methane_emission_overrides)
    }
}

pub struct UpdatedPneumaticDeviceMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticDeviceMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedPneumaticDeviceMonthMethaneEmissionOverridesLoader {
    type Value = Vec<PneumaticDeviceMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_device_month_methane_emission_overrides = query_as!(
            PneumaticDeviceMonthMethaneEmissionOverride,
            r#"SELECT * FROM pneumatic_device_month_methane_emission_override WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_device_month_methane_emission_overrides.sort_by_key(
            |pneumatic_device_month_methane_emission_override| {
                pneumatic_device_month_methane_emission_override.updated_by_id
            },
        );

        let updated_pneumatic_device_month_methane_emission_overrides =
            pneumatic_device_month_methane_emission_overrides
                .into_iter()
                .group_by(|pneumatic_device_month_methane_emission_override| {
                    pneumatic_device_month_methane_emission_override.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(updated_pneumatic_device_month_methane_emission_overrides)
    }
}

pub struct PneumaticDeviceMonthMethaneEmissionOverridesByPneumaticDeviceLoader {
    pool: Data<PgPool>,
}

impl PneumaticDeviceMonthMethaneEmissionOverridesByPneumaticDeviceLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticDeviceMonthMethaneEmissionOverridesByPneumaticDeviceLoader {
    type Value = Vec<PneumaticDeviceMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_device_month_methane_emission_overrides = query_as!(
            PneumaticDeviceMonthMethaneEmissionOverride,
            r#"SELECT * FROM pneumatic_device_month_methane_emission_override WHERE pneumatic_device_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_device_month_methane_emission_overrides.sort_by_key(
            |pneumatic_device_month_methane_emission_override| {
                pneumatic_device_month_methane_emission_override.pneumatic_device_id
            },
        );

        let pneumatic_device_month_methane_emission_overrides =
            pneumatic_device_month_methane_emission_overrides
                .into_iter()
                .group_by(|pneumatic_device_month_methane_emission_override| {
                    pneumatic_device_month_methane_emission_override.pneumatic_device_id
                })
                .into_iter()
                .map(|(pneumatic_device_id, group)| (pneumatic_device_id, group.collect()))
                .collect();

        Ok(pneumatic_device_month_methane_emission_overrides)
    }
}
