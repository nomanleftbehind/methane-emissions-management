use super::{super::super::super::user::User, StorageTankControlledCharacterization};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        routine::defined_vent_gas::storage_tank::StorageTankControlledCharacterizationLoader,
        user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use common::ControlDeviceInactivityReason;
use sqlx::FromRow;
use uuid::Uuid;

/// Object used to abstract periods of inactivity of control devices used to control emissions from controlled storage tanks, rendering emissions nonroutine or fugitive during those periods as described in AER Manual 015 section [`4.2.2`](https://static.aer.ca/prd/documents/manuals/Manual015.pdf#page=29).
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct StorageTankControlDeviceInactivity {
    pub id: Uuid,
    pub storage_tank_controlled_characterization_id: Uuid,
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
impl StorageTankControlDeviceInactivity {
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

    async fn storage_tank_controlled_characterization(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<StorageTankControlledCharacterization>, Error> {
        let loader = ctx.get_loader::<DataLoader<StorageTankControlledCharacterizationLoader>>();
        let storage_tank_controlled_characterization = loader
            .load_one(self.storage_tank_controlled_characterization_id)
            .await;

        storage_tank_controlled_characterization
    }
}
