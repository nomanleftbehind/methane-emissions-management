use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        controller_manufacturer_loader::ControllerManufacturerLoader, user_loader::UserLoader,
    },
    models::{pneumatic_device::ControllerManufacturer, User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct LevelController {
    pub id: Uuid,
    pub site_id: Uuid,
    pub manufacturer_id: Uuid,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl LevelController {
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

    async fn manufacturer(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<ControllerManufacturer>, Error> {
        let loader = ctx.get_loader::<DataLoader<ControllerManufacturerLoader>>();
        let manufacturer = loader.load_one(self.manufacturer_id).await;

        manufacturer
    }
}
