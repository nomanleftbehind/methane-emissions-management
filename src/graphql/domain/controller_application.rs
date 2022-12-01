use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        controller_loader::ControllersByApplicationLoader, user_loader::UserLoader,
    },
    domain::{Controller, User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Debug, Clone, FromRow, PartialEq)]
#[graphql(complex)]
pub struct ControllerApplication {
    pub id: Uuid,
    pub application: String,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl ControllerApplication {
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

    async fn controllers(&self, ctx: &Context<'_>) -> Result<Vec<Controller>, Error> {
        let loader = ctx.get_loader::<DataLoader<ControllersByApplicationLoader>>();
        let controllers = loader.load_one(self.id).await?;
        let result = controllers.unwrap_or(vec![]);

        Ok(result)
    }
}
