use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        facility_loader::FacilityLoader,
        gas_analysis_calculated_param_loader::GasAnalysisCalculatedParamLoader,
        user_loader::UserLoader,
    },
    models::{Facility, GasAnalysisCalculatedParam, User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct GasAnalysis {
    pub id: Uuid,
    pub facility_id: Uuid,
    pub date: NaiveDate,
    pub h2: f64,
    pub he: f64,
    pub n2: f64,
    pub co2: f64,
    pub h2s: f64,
    pub c1: f64,
    pub c2: f64,
    pub c3: f64,
    pub c4_i: f64,
    pub c4_n: f64,
    pub c5_i: f64,
    pub c5_n: f64,
    pub c6: f64,
    pub c7_plus: f64,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl GasAnalysis {
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

    pub async fn facility(&self, ctx: &Context<'_>) -> Result<Option<Facility>, Error> {
        let loader = ctx.get_loader::<DataLoader<FacilityLoader>>();
        let facility = loader.load_one(self.facility_id).await;

        facility
    }

    pub async fn gas_analysis_calculated_param(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<GasAnalysisCalculatedParam>, Error> {
        let loader = ctx.get_loader::<DataLoader<GasAnalysisCalculatedParamLoader>>();
        let gas_analysis_calculated_param = loader.load_one(self.id).await;

        gas_analysis_calculated_param
    }
}
