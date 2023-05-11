use super::StorageTank;
use crate::graphql::{
    context::ContextExt,
    dataloaders::{defined_vent_gas::storage_tank::StorageTankLoader, user::UserLoader},
    models::User,
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use common::ControlledCharacterization;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct StorageTankControlledCharacterization {
    pub id: Uuid,
    pub storage_tank_id: Uuid,
    pub date: NaiveDate,
    pub controlled_characterization: ControlledCharacterization,
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
}
