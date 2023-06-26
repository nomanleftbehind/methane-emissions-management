use super::{super::super::super::user::User, PneumaticPump, PneumaticPumpControlDeviceInactivity};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        routine::pneumatic_device::pneumatic_pump::{
            PneumaticPumpControlDeviceInactivitiesByPneumaticPumpControlledCharacterizationLoader,
            PneumaticPumpLoader,
        },
        user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use common::ControlDevice;
use sqlx::FromRow;
use uuid::Uuid;

/// Object representing changes to controlled or uncontrolled characterization of pneumatic pump as defined in AER Manual 015 section [`1.1.2`](https://static.aer.ca/prd/documents/manuals/Manual015.pdf#page=10).
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct PneumaticPumpControlledCharacterization {
    pub id: Uuid,
    pub pneumatic_pump_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub control_device: ControlDevice,
    pub comment: Option<String>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl PneumaticPumpControlledCharacterization {
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

    async fn pneumatic_pump(&self, ctx: &Context<'_>) -> Result<Option<PneumaticPump>, Error> {
        let loader = ctx.get_loader::<DataLoader<PneumaticPumpLoader>>();
        let pneumatic_pump = loader.load_one(self.pneumatic_pump_id).await;

        pneumatic_pump
    }

    async fn pneumatic_pump_control_device_inactivities(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticPumpControlDeviceInactivity>, Error> {
        let loader = ctx.get_loader::<DataLoader<
            PneumaticPumpControlDeviceInactivitiesByPneumaticPumpControlledCharacterizationLoader,
        >>();
        let pneumatic_pump_control_device_inactivities = loader.load_one(self.id).await?;
        let result = pneumatic_pump_control_device_inactivities.unwrap_or(vec![]);

        Ok(result)
    }
}
