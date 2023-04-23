use super::CompressorSeal;
use crate::graphql::{
    context::ContextExt,
    dataloaders::{compressor::CompressorSealLoader, user::UserLoader},
    models::user::User,
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct CompressorSealTest {
    pub id: Uuid,
    pub compressor_seal_id: Uuid,
    pub date: NaiveDate,
    pub rate: f64,
    pub survey_equipment_id: Uuid,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl CompressorSealTest {
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

    async fn compressor_seal(&self, ctx: &Context<'_>) -> Result<Option<CompressorSeal>, Error> {
        let loader = ctx.get_loader::<DataLoader<CompressorSealLoader>>();
        let compressor_seal = loader.load_one(self.compressor_seal_id).await;

        compressor_seal
    }
}
