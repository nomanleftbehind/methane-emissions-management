use super::{
    super::super::{month_methane_emission::MonthMethaneEmission, site::Site, user::User},
    StorageTankChange, StorageTankGasInSolutionFactorCalculated,
    StorageTankMonthLiquidHydrocarbonEntering, StorageTankMonthMethaneEmissionOverride,
};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        defined_vent_gas::storage_tank::{
            StorageTankChangesByStorageTankLoader,
            StorageTankGasInSolutionFactorsCalculatedByStorageTankLoader,
            StorageTankMonthLiquidHydrocarbonEnteringByStorageTankLoader,
            StorageTankMonthMethaneEmissionOverridesByStorageTankLoader,
        },
        month_methane_emission::MonthMethaneEmissionsBySourceTableLoader,
        site::SiteLoader,
        user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct StorageTank {
    pub id: Uuid,
    pub site_id: Uuid,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl StorageTank {
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

    async fn storage_tank_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<StorageTankChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<StorageTankChangesByStorageTankLoader>>();
        let storage_tank_changes = loader.load_one(self.id).await?;
        let result = storage_tank_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn storage_tank_month_liquid_hydrocarbon_entering(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<StorageTankMonthLiquidHydrocarbonEntering>, Error> {
        let loader = ctx
            .get_loader::<DataLoader<StorageTankMonthLiquidHydrocarbonEnteringByStorageTankLoader>>(
            );
        let storage_tank_month_liquid_hydrocarbon_entering = loader.load_one(self.id).await?;
        let result = storage_tank_month_liquid_hydrocarbon_entering.unwrap_or(vec![]);

        Ok(result)
    }

    async fn storage_tank_gas_in_solution_factors_calculated(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<StorageTankGasInSolutionFactorCalculated>, Error> {
        let loader = ctx
            .get_loader::<DataLoader<StorageTankGasInSolutionFactorsCalculatedByStorageTankLoader>>(
            );
        let storage_tank_gas_in_solution_factors_calculated = loader.load_one(self.id).await?;
        let result = storage_tank_gas_in_solution_factors_calculated.unwrap_or(vec![]);

        Ok(result)
    }

    async fn storage_tank_month_methane_emission_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<StorageTankMonthMethaneEmissionOverride>, Error> {
        let loader = ctx
            .get_loader::<DataLoader<StorageTankMonthMethaneEmissionOverridesByStorageTankLoader>>(
            );
        let storage_tank_month_methane_emission_overrides = loader.load_one(self.id).await?;
        let result = storage_tank_month_methane_emission_overrides.unwrap_or(vec![]);

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
