use crate::graphql::{
    context::ContextExt,
    dataloaders::{compressor::CompressorSealTestsBySurveyEquipmentLoader, user::UserLoader},
    models::{compressor::CompressorSealTest, user::User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct SurveyEquipment {
    pub id: Uuid,
    pub make: String,
    pub model: String,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl SurveyEquipment {
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

    async fn compressor_seal_tests(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorSealTest>, Error> {
        let loader = ctx.get_loader::<DataLoader<CompressorSealTestsBySurveyEquipmentLoader>>();
        let compressor_seal_tests = loader.load_one(self.id).await?;
        let result = compressor_seal_tests.unwrap_or(vec![]);

        Ok(result)
    }
}
