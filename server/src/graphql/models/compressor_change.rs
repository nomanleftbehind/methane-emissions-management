use crate::graphql::{
    context::ContextExt,
    dataloaders::{compressor_loader::CompressorLoader, user_loader::UserLoader},
    models::{Compressor, User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Enum, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Enum, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Debug, sqlx::Type)]
#[sqlx(type_name = "calculation_method", rename_all = "UPPERCASE")]
pub enum CalculationMethod {
    Equation,
    Measured,
}

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct CompressorChange {
    pub id: Uuid,
    pub compressor_id: Uuid,
    pub date: NaiveDate,
    pub calculation_method: CalculationMethod,
    pub number_of_throws: i32,
    pub rate: f64,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl CompressorChange {
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

    async fn controller(&self, ctx: &Context<'_>) -> Result<Option<Compressor>, Error> {
        let loader = ctx.get_loader::<DataLoader<CompressorLoader>>();
        let controller = loader.load_one(self.compressor_id).await;

        controller
    }
}
