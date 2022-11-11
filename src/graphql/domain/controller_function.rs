use crate::graphql::{
    context::ContextExt,
    dataloaders::{ControllerFunctionControllersLoader, UserLoader},
    domain::{Controller, User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use sqlx::{types::time::PrimitiveDateTime, FromRow};
use uuid::Uuid;

#[derive(SimpleObject, Debug, Clone, FromRow, PartialEq)]
#[graphql(complex)]
pub struct ControllerFunction {
    pub id: Uuid,
    pub function: String,
    pub created_by_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: PrimitiveDateTime,
}

#[ComplexObject]
impl ControllerFunction {
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
        let loader = ctx.get_loader::<DataLoader<ControllerFunctionControllersLoader>>();
        let controllers = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no written controllers
        let result = controllers.unwrap_or(vec![]);

        Ok(result)
    }
}
