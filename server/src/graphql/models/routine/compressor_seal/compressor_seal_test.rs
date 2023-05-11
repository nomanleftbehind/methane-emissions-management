use super::{super::super::user::User, CompressorSeal};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{routine::compressor_seal::CompressorSealLoader, user::UserLoader},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use common::CompressorSealTestingPoint;
use sqlx::FromRow;
use uuid::Uuid;

/// Object representing requirement in [`AER Directive 060 section 8.6.2.1`](https://static.aer.ca/prd/documents/directives/Directive060.pdf#page=75).
///
/// Rate represents methane leak expressed in standard cubic feet per hour.
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct CompressorSealTest {
    pub id: Uuid,
    pub compressor_seal_id: Uuid,
    pub date: NaiveDate,
    /// standard cubic feet per hour of methane
    pub rate: f64,
    pub testing_point: CompressorSealTestingPoint,
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
