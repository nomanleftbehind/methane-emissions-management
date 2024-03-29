use super::{CompressorSealMonthMethaneEmissionOverride, CompressorSealTest};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        month_methane_emission::MonthMethaneEmissionsBySourceTableLoader,
        routine::compressor_seal::{
            CompressorSealMonthMethaneEmissionOverridesByCompressorSealLoader,
            CompressorSealTestsByCompressorSealLoader,
        },
        user::UserLoader,
    },
    models::{month_methane_emission::MonthMethaneEmission, user::User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::NaiveDateTime;
use common::SealType;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct CompressorSeal {
    pub id: Uuid,
    pub r#type: SealType,
    pub description: Option<String>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl CompressorSeal {
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
        let loader = ctx.get_loader::<DataLoader<CompressorSealTestsByCompressorSealLoader>>();
        let compressor_month_vent_overrides = loader.load_one(self.id).await?;
        let result = compressor_month_vent_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn compressor_seal_month_methane_emission_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorSealMonthMethaneEmissionOverride>, Error> {
        let loader = ctx.get_loader::<DataLoader<CompressorSealMonthMethaneEmissionOverridesByCompressorSealLoader>>();
        let compressor_seal_month_methane_emission_overrides = loader.load_one(self.id).await?;
        let result = compressor_seal_month_methane_emission_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn month_methane_emissions(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<MonthMethaneEmission>, Error> {
        let loader = ctx.get_loader::<DataLoader<MonthMethaneEmissionsBySourceTableLoader>>();
        let controller_month_vents = loader.load_one(self.id).await?;
        let result = controller_month_vents.unwrap_or(vec![]);

        Ok(result)
    }
}
