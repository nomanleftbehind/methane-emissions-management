use crate::graphql::{context::ContextExt, dataloaders::UserLoader, domain::User};
use async_graphql::dataloader::DataLoader;
use async_graphql::*;
use sqlx::{types::time::PrimitiveDateTime, FromRow};
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct Controller {
    pub id: Uuid,
    pub fdc_rec_id: String,
    pub manufacturer_id: Uuid,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub function_id: Option<Uuid>,
    pub facility_id: Uuid,
    pub created_by_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: PrimitiveDateTime,
}

#[ComplexObject]
impl Controller {
    async fn created_by(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let created_by = loader.load_one(self.created_by_id).await;

        created_by
    }

    async fn updated_by(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let updated_by = loader.load_one(self.updated_by_id).await;

        updated_by
    }
}
