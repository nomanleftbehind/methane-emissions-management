use super::{
    super::{
        super::super::{month_methane_emission::MonthMethaneEmission, site::Site, user::User},
        DeviceManufacturer,
    },
    PneumaticInstrumentChange, PneumaticInstrumentMonthHours,
    PneumaticInstrumentMonthMethaneEmissionOverride,
};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        month_methane_emission::MonthMethaneEmissionsBySourceTableLoader,
        routine::pneumatic_device::{
            pneumatic_instrument::{
                PneumaticInstrumentChangesByPneumaticInstrumentLoader,
                PneumaticInstrumentMonthHoursByPneumaticInstrumentLoader,
                PneumaticInstrumentMonthMethaneEmissionOverridesByPneumaticInstrumentLoader,
            },
            DeviceManufacturerLoader,
        },
        site::SiteLoader,
        user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use common::PneumaticInstrumentType;
use sqlx::FromRow;
use uuid::Uuid;

/// Pneumatic pump or any pneumatic instrument excluding level controller, as defined in AER Directive 060 [`Appendix 2`](https://static.aer.ca/prd/documents/directives/Directive060.pdf#page=98).
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct PneumaticInstrument {
    pub id: Uuid,
    pub site_id: Uuid,
    pub r#type: PneumaticInstrumentType,
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
impl PneumaticInstrument {
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

    async fn pneumatic_instrument_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticInstrumentChange>, Error> {
        let loader =
            ctx.get_loader::<DataLoader<PneumaticInstrumentChangesByPneumaticInstrumentLoader>>();
        let pneumatic_instrument_changes = loader.load_one(self.id).await?;
        let result = pneumatic_instrument_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn pneumatic_instrument_month_hours(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticInstrumentMonthHours>, Error> {
        let loader = ctx
            .get_loader::<DataLoader<PneumaticInstrumentMonthHoursByPneumaticInstrumentLoader>>();
        let pneumatic_instrument_month_hours = loader.load_one(self.id).await?;
        let result = pneumatic_instrument_month_hours.unwrap_or(vec![]);

        Ok(result)
    }

    async fn pneumatic_instrument_month_methane_emission_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticInstrumentMonthMethaneEmissionOverride>, Error> {
        let loader =
            ctx.get_loader::<DataLoader<
                PneumaticInstrumentMonthMethaneEmissionOverridesByPneumaticInstrumentLoader,
            >>();
        let pneumatic_instrument_month_methane_emission_overrides =
            loader.load_one(self.id).await?;
        let result = pneumatic_instrument_month_methane_emission_overrides.unwrap_or(vec![]);

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
