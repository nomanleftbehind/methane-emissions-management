use crate::graphql::{
    context::ContextExt,
    dataloaders::{compressor::CompressorLoader, user::UserLoader},
    models::{compressor::Compressor, user::User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

/// Model representing user overrides of calculated monthly methane emission volumes from compressor seals.
///
/// Field `month` is a [`NaiveDate`](chrono::NaiveDate), which must be first day of the month. This is impossible to enforce on database level, but is instead guaranteed through [`MonthBeginningValidator`](crate::graphql::models::validators::MonthBeginningValidator).
///
/// Field `gas_volume` is in m³.
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct CompressorSealMonthMethaneEmissionOverride {
    pub id: Uuid,
    pub compressor_seal_id: Uuid,
    pub month: NaiveDate,
    /// m³ per month of natural gas
    pub gas_volume: f64,
    pub comment: Option<String>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl CompressorSealMonthMethaneEmissionOverride {
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
        let compressor = loader.load_one(self.compressor_seal_id).await;

        compressor
    }
}
