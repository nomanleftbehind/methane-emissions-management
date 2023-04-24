use super::{LevelControllerActuationFrequency, PneumaticDeviceChange};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        month_methane_emission::MonthMethaneEmissionsByEmissionSourceLoader,
        pneumatic_device::{
            DeviceManufacturerLoader, LevelControllerActuationFrequenciesByLevelControllerLoader,
            PneumaticDeviceChangesByPneumaticDeviceLoader,
            PneumaticDeviceMonthHoursByPneumaticDeviceLoader,
            PneumaticDeviceMonthMethaneEmissionOverridesByPneumaticDeviceLoader,
        },
        site::SiteLoader,
        user::UserLoader,
    },
    models::{
        month_methane_emission::MonthMethaneEmission,
        pneumatic_device::{
            DeviceManufacturer, PneumaticDeviceMonthHours,
            PneumaticDeviceMonthMethaneEmissionOverride,
        },
        site::Site,
        user::User,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use common::PneumaticDeviceType;
use sqlx::FromRow;
use uuid::Uuid;

/// Pneumatic instrument: A pneumatic device, powered by pressurized gas, used for maintaining a process condition such as liquid level, pressure, or temperature. Includes positioners, pressure controllers, level controllers, temperature controllers, and transducers.
///
/// or
///
/// Pneumatic pump: A pneumatic device that uses pressurized gas to move a piston or diaphragm, which pumps liquids on the opposite side of the piston or diaphragm. Includes methanol and chemical injection pumps, but does not include energy exchange pumps.
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct PneumaticDevice {
    pub id: Uuid,
    pub site_id: Uuid,
    pub r#type: PneumaticDeviceType,
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
impl PneumaticDevice {
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

    async fn pneumatic_device_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticDeviceChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<PneumaticDeviceChangesByPneumaticDeviceLoader>>();
        let pneumatic_device_changes = loader.load_one(self.id).await?;
        let result = pneumatic_device_changes.unwrap_or(vec![]);

        Ok(result)
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

    async fn pneumatic_device_month_hours(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticDeviceMonthHours>, Error> {
        let loader =
            ctx.get_loader::<DataLoader<PneumaticDeviceMonthHoursByPneumaticDeviceLoader>>();
        let pneumatic_device_month_hours = loader.load_one(self.id).await?;
        let result = pneumatic_device_month_hours.unwrap_or(vec![]);

        Ok(result)
    }

    async fn pneumatic_device_month_methane_emission_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticDeviceMonthMethaneEmissionOverride>, Error> {
        let loader = ctx.get_loader::<DataLoader<PneumaticDeviceMonthMethaneEmissionOverridesByPneumaticDeviceLoader>>();
        let pneumatic_device_month_methane_emission_overrides = loader.load_one(self.id).await?;
        let result = pneumatic_device_month_methane_emission_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn month_methane_emissions(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<MonthMethaneEmission>, Error> {
        let loader = ctx.get_loader::<DataLoader<MonthMethaneEmissionsByEmissionSourceLoader>>();
        let controller_month_vents = loader.load_one(self.id).await?;
        let result = controller_month_vents.unwrap_or(vec![]);

        Ok(result)
    }
}
