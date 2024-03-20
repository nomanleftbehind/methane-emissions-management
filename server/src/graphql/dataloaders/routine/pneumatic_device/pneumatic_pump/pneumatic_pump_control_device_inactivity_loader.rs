use crate::graphql::models::routine::pneumatic_device::pneumatic_pump::PneumaticPumpControlDeviceInactivity;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticPumpControlDeviceInactivityLoader {
    pool: Data<PgPool>,
}

impl PneumaticPumpControlDeviceInactivityLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for PneumaticPumpControlDeviceInactivityLoader {
    type Value = PneumaticPumpControlDeviceInactivity;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_pump_control_device_inactivities = query_as!(
            PneumaticPumpControlDeviceInactivity,
            r#"
            SELECT
            id, pneumatic_pump_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_pump_control_device_inactivity
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_pump_control_device_inactivity| (pneumatic_pump_control_device_inactivity.id, pneumatic_pump_control_device_inactivity))
        .collect();

        Ok(pneumatic_pump_control_device_inactivities)
    }
}

pub struct CreatedPneumaticPumpControlDeviceInactivitiesLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticPumpControlDeviceInactivitiesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedPneumaticPumpControlDeviceInactivitiesLoader {
    type Value = Vec<PneumaticPumpControlDeviceInactivity>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_control_device_inactivities = query_as!(
            PneumaticPumpControlDeviceInactivity,
            r#"
            SELECT
            id, pneumatic_pump_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_pump_control_device_inactivity
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_control_device_inactivities.sort_by_key(
            |pneumatic_pump_control_device_inactivity| {
                pneumatic_pump_control_device_inactivity.created_by_id
            },
        );

        let pneumatic_pump_control_device_inactivities = pneumatic_pump_control_device_inactivities
            .into_iter()
            .group_by(|pneumatic_pump_control_device_inactivity| {
                pneumatic_pump_control_device_inactivity.created_by_id
            })
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(pneumatic_pump_control_device_inactivities)
    }
}

pub struct UpdatedPneumaticPumpControlDeviceInactivitiesLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticPumpControlDeviceInactivitiesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedPneumaticPumpControlDeviceInactivitiesLoader {
    type Value = Vec<PneumaticPumpControlDeviceInactivity>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_control_device_inactivities = query_as!(
            PneumaticPumpControlDeviceInactivity,
            r#"
            SELECT
            id, pneumatic_pump_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_pump_control_device_inactivity
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_control_device_inactivities.sort_by_key(
            |pneumatic_pump_control_device_inactivity| {
                pneumatic_pump_control_device_inactivity.updated_by_id
            },
        );

        let pneumatic_pump_control_device_inactivities = pneumatic_pump_control_device_inactivities
            .into_iter()
            .group_by(|pneumatic_pump_control_device_inactivity| {
                pneumatic_pump_control_device_inactivity.updated_by_id
            })
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(pneumatic_pump_control_device_inactivities)
    }
}

pub struct PneumaticPumpControlDeviceInactivitiesByPneumaticPumpControlledCharacterizationLoader {
    pool: Data<PgPool>,
}

impl PneumaticPumpControlDeviceInactivitiesByPneumaticPumpControlledCharacterizationLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid>
    for PneumaticPumpControlDeviceInactivitiesByPneumaticPumpControlledCharacterizationLoader
{
    type Value = Vec<PneumaticPumpControlDeviceInactivity>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_control_device_inactivities = query_as!(
            PneumaticPumpControlDeviceInactivity,
            r#"
            SELECT
            id, pneumatic_pump_controlled_characterization_id, start_date, end_date, reason as "reason: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_pump_control_device_inactivity
            WHERE pneumatic_pump_controlled_characterization_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_control_device_inactivities.sort_by_key(
            |pneumatic_pump_control_device_inactivity| {
                pneumatic_pump_control_device_inactivity
                    .pneumatic_pump_controlled_characterization_id
            },
        );

        let pneumatic_pump_control_device_inactivities = pneumatic_pump_control_device_inactivities
            .into_iter()
            .group_by(|pneumatic_pump_control_device_inactivity| {
                pneumatic_pump_control_device_inactivity
                    .pneumatic_pump_controlled_characterization_id
            })
            .into_iter()
            .map(|(pneumatic_pump_controlled_characterization_id, group)| {
                (
                    pneumatic_pump_controlled_characterization_id,
                    group.collect(),
                )
            })
            .collect();

        Ok(pneumatic_pump_control_device_inactivities)
    }
}
