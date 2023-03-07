use crate::graphql::{
    context::ContextExt,
    dataloaders::{controller_loader::ControllerLoader, user_loader::UserLoader},
    models::{Controller, User},
};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, Error, OneofObject, SimpleObject,
};
use chrono::{NaiveDate, NaiveDateTime};
use itertools::MultiUnzip;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct ControllerMonthVent {
    pub id: Uuid,
    pub month: NaiveDate,
    pub gas_volume: f64,
    pub c1_volume: f64,
    pub co2_volume: f64,
    pub controller_id: Uuid,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl ControllerMonthVent {
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

    async fn controller(&self, ctx: &Context<'_>) -> Result<Option<Controller>, Error> {
        let loader = ctx.get_loader::<DataLoader<ControllerLoader>>();
        let controller = loader.load_one(self.controller_id).await;

        controller
    }
}

#[derive(SimpleObject, Clone, FromRow, Debug)]
pub struct ControllerMonthVentCalculated {
    pub month: NaiveDate,
    pub gas_volume: f64,
    pub c1_volume: f64,
    pub co2_volume: f64,
    pub controller_id: Uuid,
}

#[derive(Debug)]
pub struct ControllerMonthVentUnnestedRows {
    pub user_id: Uuid,
    pub controller_month_vents_calculated: Vec<ControllerMonthVentCalculated>,
}

#[derive(Debug)]
pub struct ControllerMonthVentNestedRows {
    pub id: Vec<Uuid>,
    pub month: Vec<NaiveDate>,
    pub gas_volume: Vec<f64>,
    pub c1_volume: Vec<f64>,
    pub co2_volume: Vec<f64>,
    pub controller_id: Vec<Uuid>,
    pub created_by_id: Vec<Uuid>,
    pub created_at: Vec<NaiveDateTime>,
    pub updated_by_id: Vec<Uuid>,
    pub updated_at: Vec<NaiveDateTime>,
}

impl From<ControllerMonthVentUnnestedRows> for ControllerMonthVentNestedRows {
    fn from(
        ControllerMonthVentUnnestedRows {
            user_id,
            controller_month_vents_calculated,
        }: ControllerMonthVentUnnestedRows,
    ) -> Self {
        let (
            id,
            month,
            gas_volume,
            c1_volume,
            co2_volume,
            controller_id,
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
        ) = controller_month_vents_calculated
            .into_iter()
            .map(|cmvc| {
                (
                    Uuid::new_v4(),
                    cmvc.month,
                    cmvc.gas_volume,
                    cmvc.c1_volume,
                    cmvc.co2_volume,
                    cmvc.controller_id,
                    user_id.clone(),
                    chrono::Utc::now().naive_utc(),
                    user_id.clone(),
                    chrono::Utc::now().naive_utc(),
                )
            })
            .multiunzip();

        ControllerMonthVentNestedRows {
            id,
            month,
            gas_volume,
            c1_volume,
            co2_volume,
            controller_id,
            created_by_id,
            created_at,
            updated_by_id,
            updated_at,
        }
    }
}

#[derive(Debug, OneofObject)]
pub enum ControllerMonthVentBy {
    ControllerId(Uuid),
    Month(NaiveDate),
}
