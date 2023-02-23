use crate::graphql::{
    context::ContextExt,
    dataloaders::{gas_analysis_loader::GasAnalysisLoader, user_loader::UserLoader},
    models::{GasAnalysis, User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct GasAnalysisCalculatedParam {
    pub id: Uuid,
    pub gas_gravity: f64,
    pub higher_heating_value: f64,
    pub carbon_content: f64,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl GasAnalysisCalculatedParam {
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

    async fn gas_analysis(&self, ctx: &Context<'_>) -> Result<Option<GasAnalysis>, Error> {
        let loader = ctx.get_loader::<DataLoader<GasAnalysisLoader>>();
        let gas_analysis = loader.load_one(self.id).await;

        gas_analysis
    }
}
