use super::{
    super::{
        super::{month_methane_emission::MonthMethaneEmission, site::Site, user::User},
        DeviceManufacturer,
    },
    NonLevelControllerChange, NonLevelControllerMonthHours,
    NonLevelControllerMonthMethaneEmissionOverride,
};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        month_methane_emission::MonthMethaneEmissionsBySourceTableLoader,
        pneumatic_device::{
            non_level_controller::{
                NonLevelControllerChangesByNonLevelControllerLoader,
                NonLevelControllerMonthHoursByNonLevelControllerLoader,
                NonLevelControllerMonthMethaneEmissionOverridesByNonLevelControllerLoader,
            },
            DeviceManufacturerLoader,
        },
        site::SiteLoader,
        user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use common::NonLevelControllerType;
use sqlx::FromRow;
use uuid::Uuid;

/// Pneumatic pump or any pneumatic instrument excluding level controller, as defined in AER Directive 060 [`Appendix 2`](https://static.aer.ca/prd/documents/directives/Directive060.pdf#page=98).
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct NonLevelController {
    pub id: Uuid,
    pub site_id: Uuid,
    pub r#type: NonLevelControllerType,
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
impl NonLevelController {
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

    async fn non_level_controller_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<NonLevelControllerChange>, Error> {
        let loader =
            ctx.get_loader::<DataLoader<NonLevelControllerChangesByNonLevelControllerLoader>>();
        let non_level_controller_changes = loader.load_one(self.id).await?;
        let result = non_level_controller_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn non_level_controller_month_hours(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<NonLevelControllerMonthHours>, Error> {
        let loader =
            ctx.get_loader::<DataLoader<NonLevelControllerMonthHoursByNonLevelControllerLoader>>();
        let non_level_controller_month_hours = loader.load_one(self.id).await?;
        let result = non_level_controller_month_hours.unwrap_or(vec![]);

        Ok(result)
    }

    async fn non_level_controller_month_methane_emission_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<NonLevelControllerMonthMethaneEmissionOverride>, Error> {
        let loader =
            ctx.get_loader::<DataLoader<
                NonLevelControllerMonthMethaneEmissionOverridesByNonLevelControllerLoader,
            >>();
        let non_level_controller_month_methane_emission_overrides =
            loader.load_one(self.id).await?;
        let result = non_level_controller_month_methane_emission_overrides.unwrap_or(vec![]);

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
