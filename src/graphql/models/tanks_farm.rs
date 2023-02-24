use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        facility_loader::FacilityLoader, tank_farm_change_loader::TankFarmChangesByTankFarmLoader,
        tank_farm_month_oil_flow_loader::TankFarmMonthOilFlowsByTankFarmLoader,
        tank_farm_vent_factor_loader::TankFarmVentFactorsCalculatedByTankFarmLoader,
        user_loader::UserLoader,
    },
    models::{Facility, TankFarmChange, TankFarmMonthOilFlow, TankFarmVentFactorCalculated, User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct TankFarm {
    pub id: Uuid,
    pub facility_id: Uuid,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl TankFarm {
    pub(in crate::graphql) async fn created_by(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<User>, Error> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let created_by = loader.load_one(self.created_by_id).await;

        created_by
    }

    pub(in crate::graphql) async fn updated_by(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<User>, Error> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let updated_by = loader.load_one(self.updated_by_id).await;

        updated_by
    }

    pub(in crate::graphql) async fn facility(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<Facility>, Error> {
        let loader = ctx.get_loader::<DataLoader<FacilityLoader>>();
        let facility = loader.load_one(self.facility_id).await;

        facility
    }

    async fn tank_farm_changes(&self, ctx: &Context<'_>) -> Result<Vec<TankFarmChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<TankFarmChangesByTankFarmLoader>>();
        let tank_farm_changes = loader.load_one(self.id).await?;
        let result = tank_farm_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn tank_farm_month_oil_flows(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TankFarmMonthOilFlow>, Error> {
        let loader = ctx.get_loader::<DataLoader<TankFarmMonthOilFlowsByTankFarmLoader>>();
        let tank_farm_month_oil_flows = loader.load_one(self.id).await?;
        let result = tank_farm_month_oil_flows.unwrap_or(vec![]);

        Ok(result)
    }

    async fn tank_farm_vent_factors_calculated(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TankFarmVentFactorCalculated>, Error> {
        let loader = ctx.get_loader::<DataLoader<TankFarmVentFactorsCalculatedByTankFarmLoader>>();
        let tank_farm_vent_factors_calculated = loader.load_one(self.id).await?;
        let result = tank_farm_vent_factors_calculated.unwrap_or(vec![]);

        Ok(result)
    }
}
