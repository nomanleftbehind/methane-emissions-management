use super::{super::super::super::user::User, PneumaticPumpControlledCharacterization};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        routine::pneumatic_device::pneumatic_pump::PneumaticPumpControlledCharacterizationLoader,
        user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use common::ControlDeviceInactivityReason;
use sqlx::FromRow;
use uuid::Uuid;

/// Object used to abstract periods of inactivity of control devices used to control emissions from controlled pneumatic pumps, rendering emissions nonroutine or fugitive during those periods as described in AER Manual 015 section [`1.1.2`](https://static.aer.ca/prd/documents/manuals/Manual015.pdf#page=10).
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct PneumaticPumpControlDeviceInactivity {
    pub id: Uuid,
    pub pneumatic_pump_controlled_characterization_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub reason: ControlDeviceInactivityReason,
    pub comment: Option<String>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl PneumaticPumpControlDeviceInactivity {
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

    async fn pneumatic_pump_controlled_characterization(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<PneumaticPumpControlledCharacterization>, Error> {
        let loader = ctx.get_loader::<DataLoader<PneumaticPumpControlledCharacterizationLoader>>();
        let pneumatic_pump_controlled_characterization = loader
            .load_one(self.pneumatic_pump_controlled_characterization_id)
            .await;

        pneumatic_pump_controlled_characterization
    }
}
