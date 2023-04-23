use crate::graphql::models::defined_vent_gas::tank::TankMonthOilFlow;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct TankMonthOilFlowLoader {
    pool: Data<PgPool>,
}

impl TankMonthOilFlowLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankMonthOilFlowLoader {
    type Value = TankMonthOilFlow;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tank_month_oil_flows = query_as!(
            TankMonthOilFlow,
            r#"SELECT * FROM tank_month_oil_flow WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|tank_month_oil_flow| (tank_month_oil_flow.id, tank_month_oil_flow))
        .collect();

        Ok(tank_month_oil_flows)
    }
}

pub struct CreatedTankMonthOilFlowsLoader {
    pool: Data<PgPool>,
}

impl CreatedTankMonthOilFlowsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedTankMonthOilFlowsLoader {
    type Value = Vec<TankMonthOilFlow>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_month_oil_flows = query_as!(
            TankMonthOilFlow,
            r#"SELECT * FROM tank_month_oil_flow WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_month_oil_flows.sort_by_key(|tank_month_oil_flow| tank_month_oil_flow.created_by_id);

        let tank_month_oil_flows = tank_month_oil_flows
            .into_iter()
            .group_by(|tank_month_oil_flow| tank_month_oil_flow.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(tank_month_oil_flows)
    }
}

pub struct UpdatedTankMonthOilFlowsLoader {
    pool: Data<PgPool>,
}

impl UpdatedTankMonthOilFlowsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedTankMonthOilFlowsLoader {
    type Value = Vec<TankMonthOilFlow>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_month_oil_flows = query_as!(
            TankMonthOilFlow,
            r#"SELECT * FROM tank_month_oil_flow WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_month_oil_flows.sort_by_key(|tank_month_oil_flow| tank_month_oil_flow.updated_by_id);

        let tank_month_oil_flows = tank_month_oil_flows
            .into_iter()
            .group_by(|tank_month_oil_flow| tank_month_oil_flow.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(tank_month_oil_flows)
    }
}

pub struct TankMonthOilFlowsByTankLoader {
    pool: Data<PgPool>,
}

impl TankMonthOilFlowsByTankLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankMonthOilFlowsByTankLoader {
    type Value = Vec<TankMonthOilFlow>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tank_month_oil_flows = query_as!(
            TankMonthOilFlow,
            r#"SELECT * FROM tank_month_oil_flow WHERE tank_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        tank_month_oil_flows.sort_by_key(|tank_month_oil_flow| tank_month_oil_flow.tank_id);

        let tank_month_oil_flows = tank_month_oil_flows
            .into_iter()
            .group_by(|tank_month_oil_flow| tank_month_oil_flow.tank_id)
            .into_iter()
            .map(|(tank_id, group)| (tank_id, group.collect()))
            .collect();

        Ok(tank_month_oil_flows)
    }
}
