use crate::graphql::models::routine::pneumatic_device::pneumatic_pump::PneumaticPumpControlledCharacterization;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticPumpControlledCharacterizationLoader {
    pool: Data<PgPool>,
}

impl PneumaticPumpControlledCharacterizationLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticPumpControlledCharacterizationLoader {
    type Value = PneumaticPumpControlledCharacterization;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_pump_controlled_characterizations = query_as!(
            PneumaticPumpControlledCharacterization,
            r#"
            SELECT
            id, pneumatic_pump_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_pump_controlled_characterization
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_pump_controlled_characterization| (pneumatic_pump_controlled_characterization.id, pneumatic_pump_controlled_characterization))
        .collect();

        Ok(pneumatic_pump_controlled_characterizations)
    }
}

pub struct CreatedPneumaticPumpControlledCharacterizationsLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticPumpControlledCharacterizationsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedPneumaticPumpControlledCharacterizationsLoader {
    type Value = Vec<PneumaticPumpControlledCharacterization>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_controlled_characterizations = query_as!(
            PneumaticPumpControlledCharacterization,
            r#"
            SELECT
            id, pneumatic_pump_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_pump_controlled_characterization
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_controlled_characterizations.sort_by_key(
            |pneumatic_pump_controlled_characterization| {
                pneumatic_pump_controlled_characterization.created_by_id
            },
        );

        let pneumatic_pump_controlled_characterizations =
            pneumatic_pump_controlled_characterizations
                .into_iter()
                .group_by(|cf| cf.created_by_id)
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(pneumatic_pump_controlled_characterizations)
    }
}

pub struct UpdatedPneumaticPumpControlledCharacterizationsLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticPumpControlledCharacterizationsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedPneumaticPumpControlledCharacterizationsLoader {
    type Value = Vec<PneumaticPumpControlledCharacterization>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_controlled_characterizations = query_as!(
            PneumaticPumpControlledCharacterization,
            r#"
            SELECT
            id, pneumatic_pump_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_pump_controlled_characterization
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_controlled_characterizations.sort_by_key(
            |pneumatic_pump_controlled_characterization| {
                pneumatic_pump_controlled_characterization.updated_by_id
            },
        );

        let pneumatic_pump_controlled_characterizations =
            pneumatic_pump_controlled_characterizations
                .into_iter()
                .group_by(|pneumatic_pump_controlled_characterization| {
                    pneumatic_pump_controlled_characterization.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(pneumatic_pump_controlled_characterizations)
    }
}

pub struct PneumaticPumpControlledCharacterizationsByPneumaticPumpLoader {
    pool: Data<PgPool>,
}

impl PneumaticPumpControlledCharacterizationsByPneumaticPumpLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticPumpControlledCharacterizationsByPneumaticPumpLoader {
    type Value = Vec<PneumaticPumpControlledCharacterization>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_controlled_characterizations = sqlx::query_as!(
            PneumaticPumpControlledCharacterization,
            r#"
            SELECT
            id, pneumatic_pump_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_pump_controlled_characterization
            WHERE pneumatic_pump_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_controlled_characterizations.sort_by_key(
            |pneumatic_pump_controlled_characterization| {
                pneumatic_pump_controlled_characterization.pneumatic_pump_id
            },
        );

        let pneumatic_pump_controlled_characterizations =
            pneumatic_pump_controlled_characterizations
                .into_iter()
                .group_by(|pneumatic_pump_controlled_characterization| {
                    pneumatic_pump_controlled_characterization.pneumatic_pump_id
                })
                .into_iter()
                .map(|(pneumatic_pump_id, group)| (pneumatic_pump_id, group.collect()))
                .collect();

        Ok(pneumatic_pump_controlled_characterizations)
    }
}
