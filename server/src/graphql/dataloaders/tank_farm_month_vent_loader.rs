use crate::graphql::models::TankFarmMonthVent;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct TankFarmMonthVentLoader {
    pool: Data<PgPool>,
}

impl TankFarmMonthVentLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankFarmMonthVentLoader {
    type Value = TankFarmMonthVent;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tank_farm_month_vents = sqlx::query_as!(
            TankFarmMonthVent,
            r#"SELECT * FROM tank_farm_month_vent WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|tank_farm_month_vent| (tank_farm_month_vent.id, tank_farm_month_vent))
        .collect();

        Ok(tank_farm_month_vents)
    }
}

pub struct CreatedTankFarmMonthVentsLoader {
    pool: Data<PgPool>,
}

impl CreatedTankFarmMonthVentsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedTankFarmMonthVentsLoader {
    type Value = Vec<TankFarmMonthVent>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farm_month_vents = sqlx::query_as!(
            TankFarmMonthVent,
            r#"SELECT * FROM tank_farm_month_vent WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_farm_month_vents
            .sort_by_key(|tank_farm_month_vent| tank_farm_month_vent.created_by_id);

        let created_tank_farm_month_vents = tank_farm_month_vents
            .into_iter()
            .group_by(|tank_farm_month_vent| tank_farm_month_vent.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_tank_farm_month_vents)
    }
}

pub struct UpdatedTankFarmMonthVentsLoader {
    pool: Data<PgPool>,
}

impl UpdatedTankFarmMonthVentsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedTankFarmMonthVentsLoader {
    type Value = Vec<TankFarmMonthVent>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farm_month_vents = sqlx::query_as!(
            TankFarmMonthVent,
            r#"SELECT * FROM tank_farm_month_vent WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_farm_month_vents
            .sort_by_key(|tank_farm_month_vent| tank_farm_month_vent.updated_by_id);

        let updated_tank_farm_month_vents = tank_farm_month_vents
            .into_iter()
            .group_by(|tank_farm_month_vent| tank_farm_month_vent.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_tank_farm_month_vents)
    }
}

pub struct TankFarmMonthVentsByTankFarmLoader {
    pool: Data<PgPool>,
}

impl TankFarmMonthVentsByTankFarmLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankFarmMonthVentsByTankFarmLoader {
    type Value = Vec<TankFarmMonthVent>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farm_month_vents = sqlx::query_as!(
            TankFarmMonthVent,
            r#"SELECT * FROM tank_farm_month_vent WHERE tank_farm_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_farm_month_vents.sort_by_key(|tank_farm_month_vent| tank_farm_month_vent.tank_farm_id);

        let tank_farm_month_vents_by_tank_farm = tank_farm_month_vents
            .into_iter()
            .group_by(|tank_farm_month_vent| tank_farm_month_vent.tank_farm_id)
            .into_iter()
            .map(|(tank_farm_id, group)| (tank_farm_id, group.collect()))
            .collect();

        Ok(tank_farm_month_vents_by_tank_farm)
    }
}
