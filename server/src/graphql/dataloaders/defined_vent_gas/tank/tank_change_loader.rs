use crate::graphql::models::defined_vent_gas::tank::TankChange;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct TankChangeLoader {
    pool: Data<PgPool>,
}

impl TankChangeLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankChangeLoader {
    type Value = TankChange;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tank_changes = query_as!(
            TankChange,
            r#"
            SELECT
            id, tank_id, date, ia, vru, api_density, temperature, pressure, calculation_method as "calculation_method: _", created_by_id, created_at, updated_by_id, updated_at
            FROM tank_change
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|tank_change| (tank_change.id, tank_change))
        .collect();

        Ok(tank_changes)
    }
}

pub struct CreatedTankChangesLoader {
    pool: Data<PgPool>,
}

impl CreatedTankChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedTankChangesLoader {
    type Value = Vec<TankChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_changes = query_as!(
            TankChange,
            r#"
            SELECT
            id, tank_id, date, ia, vru, api_density, temperature, pressure, calculation_method as "calculation_method: _", created_by_id, created_at, updated_by_id, updated_at
            FROM tank_change
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_changes.sort_by_key(|tank_change| tank_change.created_by_id);

        let tank_changes = tank_changes
            .into_iter()
            .group_by(|tank_change| tank_change.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(tank_changes)
    }
}

pub struct UpdatedTankChangesLoader {
    pool: Data<PgPool>,
}

impl UpdatedTankChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedTankChangesLoader {
    type Value = Vec<TankChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_changes = query_as!(
            TankChange,
            r#"
            SELECT
            id, tank_id, date, ia, vru, api_density, temperature, pressure, calculation_method as "calculation_method: _", created_by_id, created_at, updated_by_id, updated_at
            FROM tank_change
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_changes.sort_by_key(|tank_change| tank_change.updated_by_id);

        let tank_changes = tank_changes
            .into_iter()
            .group_by(|tank_change| tank_change.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(tank_changes)
    }
}

pub struct TankChangesByTankLoader {
    pool: Data<PgPool>,
}

impl TankChangesByTankLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankChangesByTankLoader {
    type Value = Vec<TankChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_changes = query_as!(
            TankChange,
            r#"
            SELECT
            id, tank_id, date, ia, vru, api_density, temperature, pressure, calculation_method as "calculation_method: _", created_by_id, created_at, updated_by_id, updated_at
            FROM tank_change
            WHERE tank_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_changes.sort_by_key(|tank_change| tank_change.tank_id);

        let tank_changes = tank_changes
            .into_iter()
            .group_by(|tank_change| tank_change.tank_id)
            .into_iter()
            .map(|(tank_id, group)| (tank_id, group.collect()))
            .collect();

        Ok(tank_changes)
    }
}
