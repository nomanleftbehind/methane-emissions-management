use super::NonLevelControllerChange;
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        controller_change_loader::ControllerChangesByControllerLoader,
        controller_manufacturer_loader::ControllerManufacturerLoader,
        controller_month_hours_loader::ControllerMonthHoursByControllerLoader,
        controller_month_vent_loader::ControllerMonthVentsByControllerLoader,
        controller_month_vent_override_loader::ControllerMonthVentOverridesByControllerLoader,
        facility::FacilityLoader, site::SiteLoader, user::UserLoader,
    },
    models::{
        pneumatic_device::{
            ControllerMonthHours, ControllerMonthVent, ControllerMonthVentOverride,
            DeviceManufacturer,
        },
        site::Site,
        Facility, User,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::NaiveDateTime;
use common::PneumaticDeviceType;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct PneumaticDevice {
    pub id: Uuid,
    pub site_id: Uuid,
    pub r#type: PneumaticDeviceType,
    pub manufacturer_id: Uuid,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl PneumaticDevice {
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

    pub(in crate::graphql) async fn site(&self, ctx: &Context<'_>) -> Result<Option<Site>, Error> {
        let loader = ctx.get_loader::<DataLoader<SiteLoader>>();
        let facility = loader.load_one(self.site_id).await;

        facility
    }

    async fn manufacturer(&self, ctx: &Context<'_>) -> Result<Option<DeviceManufacturer>, Error> {
        let loader = ctx.get_loader::<DataLoader<ControllerManufacturerLoader>>();
        let manufacturer = loader.load_one(self.manufacturer_id).await;

        manufacturer
    }

    async fn controller_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<NonLevelControllerChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<ControllerChangesByControllerLoader>>();
        let controller_changes = loader.load_one(self.id).await?;
        let result = controller_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn controller_month_hours(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerMonthHours>, Error> {
        let loader = ctx.get_loader::<DataLoader<ControllerMonthHoursByControllerLoader>>();
        let controller_month_hours = loader.load_one(self.id).await?;
        let result = controller_month_hours.unwrap_or(vec![]);

        Ok(result)
    }

    async fn controller_month_vent_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerMonthVentOverride>, Error> {
        let loader = ctx.get_loader::<DataLoader<ControllerMonthVentOverridesByControllerLoader>>();
        let controller_month_vent_overrides = loader.load_one(self.id).await?;
        let result = controller_month_vent_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn controller_month_vents(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerMonthVent>, Error> {
        let loader = ctx.get_loader::<DataLoader<ControllerMonthVentsByControllerLoader>>();
        let controller_month_vents = loader.load_one(self.id).await?;
        let result = controller_month_vents.unwrap_or(vec![]);

        Ok(result)
    }
}
