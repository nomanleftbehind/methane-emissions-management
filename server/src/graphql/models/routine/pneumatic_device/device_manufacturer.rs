use super::{
    super::super::user::User, level_controller::LevelController,
    pneumatic_instrument::PneumaticInstrument,
};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        routine::pneumatic_device::{
            level_controller::LevelControllersByManufacturerLoader,
            pneumatic_instrument::PneumaticInstrumentsByManufacturerLoader,
        },
        user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Debug, Clone, FromRow, PartialEq)]
#[graphql(complex)]
pub struct DeviceManufacturer {
    pub id: Uuid,
    pub manufacturer: String,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl DeviceManufacturer {
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

    async fn pneumatic_instruments(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticInstrument>, Error> {
        let loader = ctx.get_loader::<DataLoader<PneumaticInstrumentsByManufacturerLoader>>();
        let pneumatic_instruments = loader.load_one(self.id).await?;
        let result = pneumatic_instruments.unwrap_or(vec![]);

        Ok(result)
    }

    async fn level_controllers(&self, ctx: &Context<'_>) -> Result<Vec<LevelController>, Error> {
        let loader = ctx.get_loader::<DataLoader<LevelControllersByManufacturerLoader>>();
        let level_controllers = loader.load_one(self.id).await?;
        let result = level_controllers.unwrap_or(vec![]);

        Ok(result)
    }
}
