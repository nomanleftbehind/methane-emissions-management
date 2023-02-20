use crate::graphql::{
    context::ContextExt,
    dataloaders::{compressor_loader::CompressorLoader, user_loader::UserLoader},
    models::{Compressor, User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct CompressorMonthVent {
    pub id: Uuid,
    pub month: NaiveDate,
    pub gas_volume: f64,
    pub c1_volume: f64,
    pub co2_volume: f64,
    pub compressor_id: Uuid,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl CompressorMonthVent {
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

    async fn compressor(&self, ctx: &Context<'_>) -> Result<Option<Compressor>, Error> {
        let loader = ctx.get_loader::<DataLoader<CompressorLoader>>();
        let compressor = loader.load_one(self.compressor_id).await;

        compressor
    }
}
