use crate::graphql::{
    context::ContextExt,
    dataloaders::{pneumatic_device::PneumaticDeviceLoader, user::UserLoader},
    models::{pneumatic_device::PneumaticDevice, user::User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

/// Model representing user overrides of calculated monthly methane emission volumes from pneumatic devices.
///
/// Field `month` is a [`NaiveDate`](chrono::NaiveDate), which must be first day of the month. This is impossible to enforce on database level, but is instead guaranteed through [`MonthBeginningValidator`](crate::graphql::mutations::validators::MonthBeginningValidator).
///
/// Field `gas_volume` is in m³.
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct PneumaticDeviceMonthMethaneEmissionOverride {
    pub id: Uuid,
    pub pneumatic_device_id: Uuid,
    pub month: NaiveDate,
    pub gas_volume: f64,
    pub comment: Option<String>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl PneumaticDeviceMonthMethaneEmissionOverride {
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

    async fn pneumatic_device(&self, ctx: &Context<'_>) -> Result<Option<PneumaticDevice>, Error> {
        let loader = ctx.get_loader::<DataLoader<PneumaticDeviceLoader>>();
        let pneumatic_device = loader.load_one(self.pneumatic_device_id).await;

        pneumatic_device
    }
}
