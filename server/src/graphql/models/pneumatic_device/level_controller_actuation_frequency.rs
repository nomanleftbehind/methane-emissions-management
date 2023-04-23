use crate::graphql::{
    context::ContextExt,
    dataloaders::{pneumatic_device::PneumaticDeviceLoader, user::UserLoader},
    models::{pneumatic_device::PneumaticDevice, user::User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct LevelControllerActuationFrequency {
    pub id: Uuid,
    pub pneumatic_device_id: Uuid,
    pub date: NaiveDate,
    /// Time between actuations in minutes
    pub actuation_frequency: f64,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl LevelControllerActuationFrequency {
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

    async fn level_controller(&self, ctx: &Context<'_>) -> Result<Option<PneumaticDevice>, Error> {
        let loader = ctx.get_loader::<DataLoader<PneumaticDeviceLoader>>();
        let level_controller = loader.load_one(self.pneumatic_device_id).await;

        level_controller
    }
}
