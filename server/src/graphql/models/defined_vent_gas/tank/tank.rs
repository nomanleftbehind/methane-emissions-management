use super::{
    TankChange, TankEmissionFactorCalculated, TankMonthMethaneEmissionOverride, TankMonthOilFlow,
};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        defined_vent_gas::tank::{
            TankChangesByTankLoader, TankEmissionFactorsCalculatedByTankLoader,
            TankMonthMethaneEmissionOverridesByTankLoader, TankMonthOilFlowsByTankLoader,
        },
        month_methane_emission::MonthMethaneEmissionsBySourceTableLoader,
        site::SiteLoader,
        user::UserLoader,
    },
    models::{site::Site, MonthMethaneEmission, User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct Tank {
    pub id: Uuid,
    pub site_id: Uuid,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Tank {
    async fn created_by(&self, ctx: &Context<'_>) -> Result<Option<User>, Error> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let created_by = loader.load_one(self.created_by_id).await;

        created_by
    }

    async fn updated_by(&self, ctx: &Context<'_>) -> Result<Option<User>, Error> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let updated_by = loader.load_one(self.updated_by_id).await;

        updated_by
    }

    async fn site(&self, ctx: &Context<'_>) -> Result<Option<Site>, Error> {
        let loader = ctx.get_loader::<DataLoader<SiteLoader>>();
        let site = loader.load_one(self.site_id).await;

        site
    }

    async fn tank_changes(&self, ctx: &Context<'_>) -> Result<Vec<TankChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<TankChangesByTankLoader>>();
        let tank_changes = loader.load_one(self.id).await?;
        let result = tank_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn tank_month_oil_flows(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TankMonthOilFlow>, Error> {
        let loader = ctx.get_loader::<DataLoader<TankMonthOilFlowsByTankLoader>>();
        let tank_month_oil_flows = loader.load_one(self.id).await?;
        let result = tank_month_oil_flows.unwrap_or(vec![]);

        Ok(result)
    }

    async fn tank_emission_factors_calculated(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TankEmissionFactorCalculated>, Error> {
        let loader = ctx.get_loader::<DataLoader<TankEmissionFactorsCalculatedByTankLoader>>();
        let tank_emission_factors_calculated = loader.load_one(self.id).await?;
        let result = tank_emission_factors_calculated.unwrap_or(vec![]);

        Ok(result)
    }

    async fn tank_month_methane_emission_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TankMonthMethaneEmissionOverride>, Error> {
        let loader = ctx.get_loader::<DataLoader<TankMonthMethaneEmissionOverridesByTankLoader>>();
        let tank_month_methane_emission_overrides = loader.load_one(self.id).await?;
        let result = tank_month_methane_emission_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn month_methane_emissions(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<MonthMethaneEmission>, Error> {
        let loader = ctx.get_loader::<DataLoader<MonthMethaneEmissionsBySourceTableLoader>>();
        let month_methane_emissions = loader.load_one(self.id).await?;
        let result = month_methane_emissions.unwrap_or(vec![]);

        Ok(result)
    }
}
