use crate::graphql::models::TankFarm;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct TankFarmLoader {
    pool: Data<PgPool>,
}

impl TankFarmLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankFarmLoader {
    type Value = TankFarm;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tank_farms = sqlx::query_as!(
            TankFarm,
            "SELECT * FROM tank_farms WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|tank_farm| (tank_farm.id, tank_farm))
        .collect();

        Ok(tank_farms)
    }
}

pub struct CreatedTankFarmsLoader {
    pool: Data<PgPool>,
}

impl CreatedTankFarmsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedTankFarmsLoader {
    type Value = Vec<TankFarm>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farms = sqlx::query_as!(
            TankFarm,
            "SELECT * FROM tank_farms WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        tank_farms.sort_by_key(|tank_farm| tank_farm.created_by_id);

        let created_tank_farms = tank_farms
            .into_iter()
            .group_by(|tank_farm| tank_farm.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_tank_farms)
    }
}

pub struct UpdatedTankFarmsLoader {
    pool: Data<PgPool>,
}

impl UpdatedTankFarmsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedTankFarmsLoader {
    type Value = Vec<TankFarm>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farms = sqlx::query_as!(
            TankFarm,
            "SELECT * FROM tank_farms WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        tank_farms.sort_by_key(|tank_farm| tank_farm.updated_by_id);

        let updated_tank_farms = tank_farms
            .into_iter()
            .group_by(|tank_farm| tank_farm.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_tank_farms)
    }
}

pub struct FacilityTankFarmLoader {
    pool: Data<PgPool>,
}

impl FacilityTankFarmLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for FacilityTankFarmLoader {
    // Equivalent loader by facility for other emitters like controllers and compressors returs Vec<Emitter> in place of value.
    // Tank farms are returned as themselves outside of vector because tank farms have one-to-one relationship with facilities,
    // unlike controllers and compressors, which have many-to-one relationship with facilities.
    type Value = TankFarm;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tank_farms = sqlx::query_as!(
            TankFarm,
            "SELECT * FROM tank_farms WHERE facility_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|tank_farm| (tank_farm.facility_id, tank_farm))
        .collect();

        Ok(tank_farms)
    }
}
