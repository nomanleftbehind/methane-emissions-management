use super::{super::super::user::User, LevelController};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{pneumatic_device::level_controller::LevelControllerLoader, user::UserLoader},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct LevelControllerChange {
    pub id: Uuid,
    pub level_controller_id: Uuid,
    pub date: NaiveDate,
    pub rate: f64,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl LevelControllerChange {
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

    async fn level_controller(&self, ctx: &Context<'_>) -> Result<Option<LevelController>, Error> {
        let loader = ctx.get_loader::<DataLoader<LevelControllerLoader>>();
        let level_controller = loader.load_one(self.level_controller_id).await;

        level_controller
    }
}
