use crate::graphql::models::TankFarmChange;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct TankFarmChangeLoader {
    pool: Data<PgPool>,
}

impl TankFarmChangeLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankFarmChangeLoader {
    type Value = TankFarmChange;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tank_farm_changes = sqlx::query_as!(
            TankFarmChange,
            r#"SELECT id, tank_farm_id, date, ia, vru, api_density, temperature, pressure, calculation_method as "calculation_method: _", created_by_id, created_at, updated_by_id, updated_at FROM tank_farm_changes WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|tank_farm_change| (tank_farm_change.id, tank_farm_change))
        .collect();

        Ok(tank_farm_changes)
    }
}

pub struct CreatedTankFarmChangesLoader {
    pool: Data<PgPool>,
}

impl CreatedTankFarmChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedTankFarmChangesLoader {
    type Value = Vec<TankFarmChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farm_changes = sqlx::query_as!(
            TankFarmChange,
            r#"SELECT id, tank_farm_id, date, ia, vru, api_density, temperature, pressure, calculation_method as "calculation_method: _", created_by_id, created_at, updated_by_id, updated_at FROM tank_farm_changes WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_farm_changes.sort_by_key(|tank_farm_change| tank_farm_change.created_by_id);

        let created_tank_farm_changes = tank_farm_changes
            .into_iter()
            .group_by(|tank_farm_change| tank_farm_change.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_tank_farm_changes)
    }
}

pub struct UpdatedTankFarmChangesLoader {
    pool: Data<PgPool>,
}

impl UpdatedTankFarmChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedTankFarmChangesLoader {
    type Value = Vec<TankFarmChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farm_changes = sqlx::query_as!(
            TankFarmChange,
            r#"SELECT id, tank_farm_id, date, ia, vru, api_density, temperature, pressure, calculation_method as "calculation_method: _", created_by_id, created_at, updated_by_id, updated_at FROM tank_farm_changes WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_farm_changes.sort_by_key(|tank_farm_change| tank_farm_change.updated_by_id);

        let updated_tank_farm_changes = tank_farm_changes
            .into_iter()
            .group_by(|tank_farm_change| tank_farm_change.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_tank_farm_changes)
    }
}

pub struct TankFarmChangesByTankFarmLoader {
    pool: Data<PgPool>,
}

impl TankFarmChangesByTankFarmLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankFarmChangesByTankFarmLoader {
    type Value = Vec<TankFarmChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farm_changes = sqlx::query_as!(
            TankFarmChange,
            r#"SELECT id, tank_farm_id, date, ia, vru, api_density, temperature, pressure, calculation_method as "calculation_method: _", created_by_id, created_at, updated_by_id, updated_at FROM tank_farm_changes WHERE tank_farm_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_farm_changes.sort_by_key(|tank_farm_change| tank_farm_change.tank_farm_id);

        let tank_farm_changes_by_tank_farm = tank_farm_changes
            .into_iter()
            .group_by(|tank_farm_change| tank_farm_change.tank_farm_id)
            .into_iter()
            .map(|(tank_farm_id, group)| (tank_farm_id, group.collect()))
            .collect();

        Ok(tank_farm_changes_by_tank_farm)
    }
}
