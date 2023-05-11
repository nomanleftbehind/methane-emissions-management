use super::{
    super::{
        super::super::{month_methane_emission::MonthMethaneEmission, site::Site, user::User},
        DeviceManufacturer,
    },
    LevelControllerActuationFrequency, LevelControllerChange,
};
use super::{LevelControllerMonthHours, LevelControllerMonthMethaneEmissionOverride};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        month_methane_emission::MonthMethaneEmissionsBySourceTableLoader,
        routine::pneumatic_device::{
            level_controller::{
                LevelControllerActuationFrequenciesByLevelControllerLoader,
                LevelControllerChangesByLevelControllerLoader,
                LevelControllerMonthHoursByLevelControllerLoader,
                LevelControllerMonthMethaneEmissionOverridesByLevelControllerLoader,
            },
            DeviceManufacturerLoader,
        },
        site::SiteLoader,
        user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

/// A pneumatic device, powered by pressurized gas, used for maintaining liquid level as defined in AER Directive 060 [`Appendix 2`](https://static.aer.ca/prd/documents/directives/Directive060.pdf#page=98).
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct LevelController {
    pub id: Uuid,
    pub site_id: Uuid,
    pub manufacturer_id: Uuid,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl LevelController {
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
        let facility = loader.load_one(self.site_id).await;

        facility
    }

    async fn manufacturer(&self, ctx: &Context<'_>) -> Result<Option<DeviceManufacturer>, Error> {
        let loader = ctx.get_loader::<DataLoader<DeviceManufacturerLoader>>();
        let manufacturer = loader.load_one(self.manufacturer_id).await;

        manufacturer
    }

    async fn level_controller_actuation_frequencies(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<LevelControllerActuationFrequency>, Error> {
        let loader = ctx
            .get_loader::<DataLoader<LevelControllerActuationFrequenciesByLevelControllerLoader>>();
        let level_controller_actuation_frequencies = loader.load_one(self.id).await?;
        let result = level_controller_actuation_frequencies.unwrap_or(vec![]);

        Ok(result)
    }

    async fn level_controller_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<LevelControllerChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<LevelControllerChangesByLevelControllerLoader>>();
        let level_controller_changes = loader.load_one(self.id).await?;
        let result = level_controller_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn level_controller_month_hours(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<LevelControllerMonthHours>, Error> {
        let loader =
            ctx.get_loader::<DataLoader<LevelControllerMonthHoursByLevelControllerLoader>>();
        let level_controller_month_hours = loader.load_one(self.id).await?;
        let result = level_controller_month_hours.unwrap_or(vec![]);

        Ok(result)
    }

    async fn level_controller_month_methane_emission_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<LevelControllerMonthMethaneEmissionOverride>, Error> {
        let loader =
            ctx.get_loader::<DataLoader<
                LevelControllerMonthMethaneEmissionOverridesByLevelControllerLoader,
            >>();
        let level_controller_month_methane_emission_overrides = loader.load_one(self.id).await?;
        let result = level_controller_month_methane_emission_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn month_methane_emissions(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<MonthMethaneEmission>, Error> {
        let loader = ctx.get_loader::<DataLoader<MonthMethaneEmissionsBySourceTableLoader>>();
        let controller_month_vents = loader.load_one(self.id).await?;
        let result = controller_month_vents.unwrap_or(vec![]);

        Ok(result)
    }
}
