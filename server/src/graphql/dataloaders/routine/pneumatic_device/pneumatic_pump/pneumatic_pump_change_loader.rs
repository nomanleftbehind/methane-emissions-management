use crate::graphql::models::routine::pneumatic_device::pneumatic_pump::PneumaticPumpChange;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticPumpChangeLoader {
    pool: Data<PgPool>,
}

impl PneumaticPumpChangeLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticPumpChangeLoader {
    type Value = PneumaticPumpChange;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_pump_changes = query_as!(
            PneumaticPumpChange,
            "SELECT * FROM pneumatic_pump_change WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_pump_change| (pneumatic_pump_change.id, pneumatic_pump_change))
        .collect();

        Ok(pneumatic_pump_changes)
    }
}

pub struct CreatedPneumaticPumpChangesLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticPumpChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedPneumaticPumpChangesLoader {
    type Value = Vec<PneumaticPumpChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_changes = query_as!(
            PneumaticPumpChange,
            "SELECT * FROM pneumatic_pump_change WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_changes
            .sort_by_key(|pneumatic_pump_change| pneumatic_pump_change.created_by_id);

        let pneumatic_pump_changes = pneumatic_pump_changes
            .into_iter()
            .group_by(|pneumatic_pump_change| pneumatic_pump_change.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(pneumatic_pump_changes)
    }
}

pub struct UpdatedPneumaticPumpChangesLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticPumpChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedPneumaticPumpChangesLoader {
    type Value = Vec<PneumaticPumpChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_changes = query_as!(
            PneumaticPumpChange,
            "SELECT * FROM pneumatic_pump_change WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_changes.sort_by_key(|pneumatic_pump| pneumatic_pump.updated_by_id);

        let pneumatic_pump_changes = pneumatic_pump_changes
            .into_iter()
            .group_by(|pneumatic_pump| pneumatic_pump.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(pneumatic_pump_changes)
    }
}

pub struct PneumaticPumpChangesByPneumaticPumpLoader {
    pool: Data<PgPool>,
}

impl PneumaticPumpChangesByPneumaticPumpLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PneumaticPumpChangesByPneumaticPumpLoader {
    type Value = Vec<PneumaticPumpChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pump_change = query_as!(
            PneumaticPumpChange,
            "SELECT * FROM pneumatic_pump_change WHERE pneumatic_pump_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        pneumatic_pump_change
            .sort_by_key(|pneumatic_pump_change| pneumatic_pump_change.pneumatic_pump_id);

        let pneumatic_pump_changes = pneumatic_pump_change
            .into_iter()
            .group_by(|pneumatic_pump_change| pneumatic_pump_change.pneumatic_pump_id)
            .into_iter()
            .map(|(pneumatic_pump_id, group)| (pneumatic_pump_id, group.collect()))
            .collect();

        Ok(pneumatic_pump_changes)
    }
}
