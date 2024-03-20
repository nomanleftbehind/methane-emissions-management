use crate::graphql::models::routine::pneumatic_device::pneumatic_instrument::PneumaticInstrumentControlDeviceInactivity;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticInstrumentControlDeviceInactivityLoader {
    pool: Data<PgPool>,
}

impl PneumaticInstrumentControlDeviceInactivityLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for PneumaticInstrumentControlDeviceInactivityLoader {
    type Value = PneumaticInstrumentControlDeviceInactivity;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_instrument_control_device_inactivities = query_as!(
            PneumaticInstrumentControlDeviceInactivity,
            r#"
            SELECT
            id, pneumatic_instrument_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument_control_device_inactivity
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_instrument_control_device_inactivity| (pneumatic_instrument_control_device_inactivity.id, pneumatic_instrument_control_device_inactivity))
        .collect();

        Ok(pneumatic_instrument_control_device_inactivities)
    }
}

pub struct CreatedPneumaticInstrumentControlDeviceInactivitiesLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticInstrumentControlDeviceInactivitiesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedPneumaticInstrumentControlDeviceInactivitiesLoader {
    type Value = Vec<PneumaticInstrumentControlDeviceInactivity>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_control_device_inactivities = query_as!(
            PneumaticInstrumentControlDeviceInactivity,
            r#"
            SELECT
            id, pneumatic_instrument_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument_control_device_inactivity
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_control_device_inactivities.sort_by_key(
            |pneumatic_instrument_control_device_inactivity| {
                pneumatic_instrument_control_device_inactivity.created_by_id
            },
        );

        let pneumatic_instrument_control_device_inactivities =
            pneumatic_instrument_control_device_inactivities
                .into_iter()
                .group_by(|pneumatic_instrument_control_device_inactivity| {
                    pneumatic_instrument_control_device_inactivity.created_by_id
                })
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(pneumatic_instrument_control_device_inactivities)
    }
}

pub struct UpdatedPneumaticInstrumentControlDeviceInactivitiesLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticInstrumentControlDeviceInactivitiesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedPneumaticInstrumentControlDeviceInactivitiesLoader {
    type Value = Vec<PneumaticInstrumentControlDeviceInactivity>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_control_device_inactivities = query_as!(
            PneumaticInstrumentControlDeviceInactivity,
            r#"
            SELECT
            id, pneumatic_instrument_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument_control_device_inactivity
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_control_device_inactivities.sort_by_key(
            |pneumatic_instrument_control_device_inactivity| {
                pneumatic_instrument_control_device_inactivity.updated_by_id
            },
        );

        let pneumatic_instrument_control_device_inactivities =
            pneumatic_instrument_control_device_inactivities
                .into_iter()
                .group_by(|pneumatic_instrument_control_device_inactivity| {
                    pneumatic_instrument_control_device_inactivity.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(pneumatic_instrument_control_device_inactivities)
    }
}

pub struct PneumaticInstrumentControlDeviceInactivitiesByPneumaticInstrumentControlledCharacterizationLoader
{
    pool: Data<PgPool>,
}

impl PneumaticInstrumentControlDeviceInactivitiesByPneumaticInstrumentControlledCharacterizationLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid>
    for PneumaticInstrumentControlDeviceInactivitiesByPneumaticInstrumentControlledCharacterizationLoader
{
    type Value = Vec<PneumaticInstrumentControlDeviceInactivity>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_control_device_inactivities = query_as!(
            PneumaticInstrumentControlDeviceInactivity,
            r#"
            SELECT
            id, pneumatic_instrument_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument_control_device_inactivity
            WHERE pneumatic_instrument_controlled_characterization_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_control_device_inactivities.sort_by_key(
            |pneumatic_instrument_control_device_inactivity| {
                pneumatic_instrument_control_device_inactivity.pneumatic_instrument_controlled_characterization_id
            },
        );

        let pneumatic_instrument_control_device_inactivities = pneumatic_instrument_control_device_inactivities
            .into_iter()
            .group_by(|pneumatic_instrument_control_device_inactivity| {
                pneumatic_instrument_control_device_inactivity.pneumatic_instrument_controlled_characterization_id
            })
            .into_iter()
            .map(|(pneumatic_instrument_controlled_characterization_id, group)| {
                (pneumatic_instrument_controlled_characterization_id, group.collect())
            })
            .collect();

        Ok(pneumatic_instrument_control_device_inactivities)
    }
}
