use super::{super::user::User, GasAnalysis};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{gas_analysis::GasAnalysisLoader, user::UserLoader},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::NaiveDateTime;
use itertools::MultiUnzip;
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

#[derive(SimpleObject, Clone, FromRow, Debug)]
pub struct GasAnalysisCalculatedParamInterim {
    pub id: Uuid,
    pub gas_gravity: f64,
    pub higher_heating_value: f64,
    pub carbon_content: f64,
}

#[derive(Debug)]
pub struct GasAnalysisCalculatedParamInterimUnnestedRows {
    pub user_id: Uuid,
    pub gas_analysis_calculated_params_interim: Vec<GasAnalysisCalculatedParamInterim>,
}

#[derive(Debug)]
pub struct GasAnalysisCalculatedParamInterimNestedRows {
    pub id: Vec<Uuid>,
    pub gas_gravity: Vec<f64>,
    pub higher_heating_value: Vec<f64>,
    pub carbon_content: Vec<f64>,
    pub created_by_id: Vec<Uuid>,
    pub created_at: Vec<NaiveDateTime>,
    pub updated_by_id: Vec<Uuid>,
    pub updated_at: Vec<NaiveDateTime>,
}

impl From<GasAnalysisCalculatedParamInterimUnnestedRows>
    for GasAnalysisCalculatedParamInterimNestedRows
{
    fn from(
        GasAnalysisCalculatedParamInterimUnnestedRows {
            user_id,
            gas_analysis_calculated_params_interim,
        }: GasAnalysisCalculatedParamInterimUnnestedRows,
    ) -> Self {
        let (
            id,
            gas_gravity,
            higher_heating_value,
            carbon_content,
            created_by_id,
            created_at,
            updated_by_id,
            updated_at,
        ): (
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
        ) = gas_analysis_calculated_params_interim
            .into_iter()
            .map(|gacp| {
                (
                    gacp.id,
                    gacp.gas_gravity,
                    gacp.higher_heating_value,
                    gacp.carbon_content,
                    user_id.clone(),
                    chrono::Utc::now().naive_utc(),
                    user_id.clone(),
                    chrono::Utc::now().naive_utc(),
                )
            })
            .multiunzip();

        GasAnalysisCalculatedParamInterimNestedRows {
            id,
            gas_gravity,
            higher_heating_value,
            carbon_content,
            created_by_id,
            created_at,
            updated_by_id,
            updated_at,
        }
    }
}
