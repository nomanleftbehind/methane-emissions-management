use crate::graphql::models::routine::pneumatic_device::pneumatic_instrument::PneumaticInstrumentControlledCharacterization;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticInstrumentControlledCharacterizationLoader {
    pool: Data<PgPool>,
}

impl PneumaticInstrumentControlledCharacterizationLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticInstrumentControlledCharacterizationLoader {
    type Value = PneumaticInstrumentControlledCharacterization;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_instrument_controlled_characterizations = query_as!(
            PneumaticInstrumentControlledCharacterization,
            r#"
            SELECT
            id, pneumatic_instrument_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument_controlled_characterization
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_instrument_controlled_characterization| (pneumatic_instrument_controlled_characterization.id, pneumatic_instrument_controlled_characterization))
        .collect();

        Ok(pneumatic_instrument_controlled_characterizations)
    }
}

pub struct CreatedPneumaticInstrumentControlledCharacterizationsLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticInstrumentControlledCharacterizationsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedPneumaticInstrumentControlledCharacterizationsLoader {
    type Value = Vec<PneumaticInstrumentControlledCharacterization>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_controlled_characterizations = query_as!(
            PneumaticInstrumentControlledCharacterization,
            r#"
            SELECT
            id, pneumatic_instrument_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument_controlled_characterization
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_controlled_characterizations.sort_by_key(
            |pneumatic_instrument_controlled_characterization| {
                pneumatic_instrument_controlled_characterization.created_by_id
            },
        );

        let pneumatic_instrument_controlled_characterizations =
            pneumatic_instrument_controlled_characterizations
                .into_iter()
                .group_by(|cf| cf.created_by_id)
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(pneumatic_instrument_controlled_characterizations)
    }
}

pub struct UpdatedPneumaticInstrumentControlledCharacterizationsLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticInstrumentControlledCharacterizationsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedPneumaticInstrumentControlledCharacterizationsLoader {
    type Value = Vec<PneumaticInstrumentControlledCharacterization>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_controlled_characterizations = query_as!(
            PneumaticInstrumentControlledCharacterization,
            r#"
            SELECT
            id, pneumatic_instrument_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument_controlled_characterization
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_controlled_characterizations.sort_by_key(
            |pneumatic_instrument_controlled_characterization| {
                pneumatic_instrument_controlled_characterization.updated_by_id
            },
        );

        let pneumatic_instrument_controlled_characterizations =
            pneumatic_instrument_controlled_characterizations
                .into_iter()
                .group_by(|pneumatic_instrument_controlled_characterization| {
                    pneumatic_instrument_controlled_characterization.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(pneumatic_instrument_controlled_characterizations)
    }
}

pub struct PneumaticInstrumentControlledCharacterizationsByPneumaticInstrumentLoader {
    pool: Data<PgPool>,
}

impl PneumaticInstrumentControlledCharacterizationsByPneumaticInstrumentLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticInstrumentControlledCharacterizationsByPneumaticInstrumentLoader {
    type Value = Vec<PneumaticInstrumentControlledCharacterization>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_controlled_characterizations = sqlx::query_as!(
            PneumaticInstrumentControlledCharacterization,
            r#"
            SELECT
            id, pneumatic_instrument_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument_controlled_characterization
            WHERE pneumatic_instrument_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_controlled_characterizations.sort_by_key(
            |pneumatic_instrument_controlled_characterization| {
                pneumatic_instrument_controlled_characterization.pneumatic_instrument_id
            },
        );

        let pneumatic_instrument_controlled_characterizations =
            pneumatic_instrument_controlled_characterizations
                .into_iter()
                .group_by(|pneumatic_instrument_controlled_characterization| {
                    pneumatic_instrument_controlled_characterization.pneumatic_instrument_id
                })
                .into_iter()
                .map(|(pneumatic_instrument_id, group)| (pneumatic_instrument_id, group.collect()))
                .collect();

        Ok(pneumatic_instrument_controlled_characterizations)
    }
}
