use crate::graphql::models::TankFarmMonthOilFlow;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct TankFarmMonthOilFlowLoader {
    pool: Data<PgPool>,
}

impl TankFarmMonthOilFlowLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankFarmMonthOilFlowLoader {
    type Value = TankFarmMonthOilFlow;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tank_farm_month_oil_flows = sqlx::query_as!(
            TankFarmMonthOilFlow,
            r#"SELECT * FROM tank_farm_month_oil_flow WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|tank_farm_month_oil_flow| (tank_farm_month_oil_flow.id, tank_farm_month_oil_flow))
        .collect();

        Ok(tank_farm_month_oil_flows)
    }
}

pub struct CreatedTankFarmMonthOilFlowsLoader {
    pool: Data<PgPool>,
}

impl CreatedTankFarmMonthOilFlowsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedTankFarmMonthOilFlowsLoader {
    type Value = Vec<TankFarmMonthOilFlow>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farm_month_oil_flows = sqlx::query_as!(
            TankFarmMonthOilFlow,
            r#"SELECT * FROM tank_farm_month_oil_flow WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_farm_month_oil_flows
            .sort_by_key(|tank_farm_month_oil_flow| tank_farm_month_oil_flow.created_by_id);

        let created_tank_farm_month_oil_flows = tank_farm_month_oil_flows
            .into_iter()
            .group_by(|tank_farm_month_oil_flow| tank_farm_month_oil_flow.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_tank_farm_month_oil_flows)
    }
}

pub struct UpdatedTankFarmMonthOilFlowsLoader {
    pool: Data<PgPool>,
}

impl UpdatedTankFarmMonthOilFlowsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedTankFarmMonthOilFlowsLoader {
    type Value = Vec<TankFarmMonthOilFlow>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farm_month_oil_flows = sqlx::query_as!(
            TankFarmMonthOilFlow,
            r#"SELECT * FROM tank_farm_month_oil_flow WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_farm_month_oil_flows
            .sort_by_key(|tank_farm_month_oil_flow| tank_farm_month_oil_flow.updated_by_id);

        let updated_tank_farm_month_oil_flows = tank_farm_month_oil_flows
            .into_iter()
            .group_by(|tank_farm_month_oil_flow| tank_farm_month_oil_flow.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_tank_farm_month_oil_flows)
    }
}

pub struct TankFarmMonthOilFlowsByTankFarmLoader {
    pool: Data<PgPool>,
}

impl TankFarmMonthOilFlowsByTankFarmLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankFarmMonthOilFlowsByTankFarmLoader {
    type Value = Vec<TankFarmMonthOilFlow>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_farm_month_oil_flows = sqlx::query_as!(
            TankFarmMonthOilFlow,
            r#"SELECT * FROM tank_farm_month_oil_flow WHERE tank_farm_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_farm_month_oil_flows
            .sort_by_key(|tank_farm_month_oil_flow| tank_farm_month_oil_flow.tank_farm_id);

        let tank_farm_month_oil_flows_by_tank_farm = tank_farm_month_oil_flows
            .into_iter()
            .group_by(|tank_farm_month_oil_flow| tank_farm_month_oil_flow.tank_farm_id)
            .into_iter()
            .map(|(tank_farm_id, group)| (tank_farm_id, group.collect()))
            .collect();

        Ok(tank_farm_month_oil_flows_by_tank_farm)
    }
}
