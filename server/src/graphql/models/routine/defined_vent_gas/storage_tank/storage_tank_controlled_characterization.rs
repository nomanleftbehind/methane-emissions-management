use super::{super::super::super::user::User, StorageTank, StorageTankControlDeviceInactivity};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        routine::defined_vent_gas::storage_tank::{
            StorageTankControlDeviceInactivitiesByStorageTankControlledCharacterizationLoader,
            StorageTankLoader,
        },
        user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use common::ControlDevice;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct StorageTankControlledCharacterization {
    pub id: Uuid,
    pub storage_tank_id: Uuid,
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
impl StorageTankControlledCharacterization {
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

    async fn storage_tank(&self, ctx: &Context<'_>) -> Result<Option<StorageTank>, Error> {
        let loader = ctx.get_loader::<DataLoader<StorageTankLoader>>();
        let storage_tank = loader.load_one(self.storage_tank_id).await;

        storage_tank
    }

    async fn storage_tank_control_device_inactivities(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<StorageTankControlDeviceInactivity>, Error> {
        let loader = ctx.get_loader::<DataLoader<
            StorageTankControlDeviceInactivitiesByStorageTankControlledCharacterizationLoader,
        >>();
        let storage_tank_control_device_inactivities = loader.load_one(self.id).await?;
        let result = storage_tank_control_device_inactivities.unwrap_or(vec![]);

        Ok(result)
    }
}
