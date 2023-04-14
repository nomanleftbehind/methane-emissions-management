use crate::graphql::{
    context::ContextExt,
    dataloaders::{controller_loader::ControllerLoader, user_loader::UserLoader},
    models::{pneumatic_device::NonLevelController, User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct ControllerMonthHours {
    pub id: Uuid,
    pub month: NaiveDate,
    pub hours_on: f64,
    pub controller_id: Uuid,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl ControllerMonthHours {
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

    async fn controller(&self, ctx: &Context<'_>) -> Result<Option<NonLevelController>, Error> {
        let loader = ctx.get_loader::<DataLoader<ControllerLoader>>();
        let controller = loader.load_one(self.controller_id).await;

        controller
    }
}
