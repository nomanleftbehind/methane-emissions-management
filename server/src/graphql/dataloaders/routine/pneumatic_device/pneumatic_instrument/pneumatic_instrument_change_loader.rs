use crate::graphql::models::routine::pneumatic_device::pneumatic_instrument::PneumaticInstrumentChange;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticInstrumentChangeLoader {
    pool: Data<PgPool>,
}

impl PneumaticInstrumentChangeLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticInstrumentChangeLoader {
    type Value = PneumaticInstrumentChange;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_instrument_changes = query_as!(
            PneumaticInstrumentChange,
            "SELECT * FROM pneumatic_instrument_change WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_instrument_change| {
            (pneumatic_instrument_change.id, pneumatic_instrument_change)
        })
        .collect();

        Ok(pneumatic_instrument_changes)
    }
}

pub struct CreatedPneumaticInstrumentChangesLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticInstrumentChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedPneumaticInstrumentChangesLoader {
    type Value = Vec<PneumaticInstrumentChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_changes = query_as!(
            PneumaticInstrumentChange,
            "SELECT * FROM pneumatic_instrument_change WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_changes
            .sort_by_key(|pneumatic_instrument_change| pneumatic_instrument_change.created_by_id);

        let pneumatic_instrument_changes = pneumatic_instrument_changes
            .into_iter()
            .group_by(|pneumatic_instrument_change| pneumatic_instrument_change.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(pneumatic_instrument_changes)
    }
}

pub struct UpdatedPneumaticInstrumentChangesLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticInstrumentChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedPneumaticInstrumentChangesLoader {
    type Value = Vec<PneumaticInstrumentChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_changes = query_as!(
            PneumaticInstrumentChange,
            "SELECT * FROM pneumatic_instrument_change WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_changes
            .sort_by_key(|pneumatic_instrument| pneumatic_instrument.updated_by_id);

        let pneumatic_instrument_changes = pneumatic_instrument_changes
            .into_iter()
            .group_by(|pneumatic_instrument| pneumatic_instrument.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(pneumatic_instrument_changes)
    }
}

pub struct PneumaticInstrumentChangesByPneumaticInstrumentLoader {
    pool: Data<PgPool>,
}

impl PneumaticInstrumentChangesByPneumaticInstrumentLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticInstrumentChangesByPneumaticInstrumentLoader {
    type Value = Vec<PneumaticInstrumentChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instrument_change = query_as!(
            PneumaticInstrumentChange,
            "SELECT * FROM pneumatic_instrument_change WHERE pneumatic_instrument_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_instrument_change.sort_by_key(|pneumatic_instrument_change| {
            pneumatic_instrument_change.pneumatic_instrument_id
        });

        let pneumatic_instrument_changes = pneumatic_instrument_change
            .into_iter()
            .group_by(|pneumatic_instrument_change| {
                pneumatic_instrument_change.pneumatic_instrument_id
            })
            .into_iter()
            .map(|(pneumatic_instrument_id, group)| (pneumatic_instrument_id, group.collect()))
            .collect();

        Ok(pneumatic_instrument_changes)
    }
}
