use crate::graphql::{
    context::ContextExt,
    dataloaders::{compressor_loader::CompressorLoader, user_loader::UserLoader},
    models::{Compressor, User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use itertools::MultiUnzip;
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

#[derive(SimpleObject, Clone, FromRow, Debug)]
pub struct CompressorMonthVentCalculated {
    pub month: NaiveDate,
    pub gas_volume: f64,
    pub c1_volume: f64,
    pub co2_volume: f64,
    pub compressor_id: Uuid,
}

#[derive(Debug)]
pub struct CompressorMonthVentUnnestedRows {
    pub user_id: Uuid,
    pub compressor_month_vents_calculated: Vec<CompressorMonthVentCalculated>,
}

#[derive(Debug)]
pub struct CompressorMonthVentNestedRows {
    pub id: Vec<Uuid>,
    pub month: Vec<NaiveDate>,
    pub gas_volume: Vec<f64>,
    pub c1_volume: Vec<f64>,
    pub co2_volume: Vec<f64>,
    pub compressor_id: Vec<Uuid>,
    pub created_by_id: Vec<Uuid>,
    pub created_at: Vec<NaiveDateTime>,
    pub updated_by_id: Vec<Uuid>,
    pub updated_at: Vec<NaiveDateTime>,
}

impl From<CompressorMonthVentUnnestedRows> for CompressorMonthVentNestedRows {
    fn from(
        CompressorMonthVentUnnestedRows {
            user_id,
            compressor_month_vents_calculated,
        }: CompressorMonthVentUnnestedRows,
    ) -> Self {
        let (
            id,
            month,
            gas_volume,
            c1_volume,
            co2_volume,
            compressor_id,
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
            Vec<_>,
            Vec<_>,
        ) = compressor_month_vents_calculated
            .into_iter()
            .map(|cmvc| {
                (
                    Uuid::new_v4(),
                    cmvc.month,
                    cmvc.gas_volume,
                    cmvc.c1_volume,
                    cmvc.co2_volume,
                    cmvc.compressor_id,
                    user_id.clone(),
                    chrono::Utc::now().naive_utc(),
                    user_id.clone(),
                    chrono::Utc::now().naive_utc(),
                )
            })
            .multiunzip();

        CompressorMonthVentNestedRows {
            id,
            month,
            gas_volume,
            c1_volume,
            co2_volume,
            compressor_id,
            created_by_id,
            created_at,
            updated_by_id,
            updated_at,
        }
    }
}
