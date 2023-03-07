use crate::graphql::{
    context::ContextExt,
    dataloaders::{tank_farm_loader::TankFarmLoader, user_loader::UserLoader},
    models::{TankFarm, User},
};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, Error, OneofObject, SimpleObject,
};
use chrono::{NaiveDate, NaiveDateTime};
use itertools::MultiUnzip;
use sqlx::FromRow;
use uuid::Uuid;

/// Model representing monthly vented volumes from tank farms.
///
/// No mutations are provided which would enable users to enter values directly, instead, all fields are calculated and writter to the database.
///
/// Field `month` has to be first day of the month. This is impossible to enforce on database level, but is instead guaranteed through [`MonthBeginningValidator`](crate::graphql::mutations::validators::MonthBeginningValidator).
///
/// All volumes are in m³.
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct TankFarmMonthVent {
    pub id: Uuid,
    pub tank_farm_id: Uuid,
    pub month: NaiveDate,
    pub gas_volume: f64,
    pub c1_volume: f64,
    pub co2_volume: f64,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl TankFarmMonthVent {
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

    async fn tank_farm(&self, ctx: &Context<'_>) -> Result<Option<TankFarm>, Error> {
        let loader = ctx.get_loader::<DataLoader<TankFarmLoader>>();
        let tank_farm = loader.load_one(self.tank_farm_id).await;

        tank_farm
    }
}

#[derive(SimpleObject, Clone, FromRow, Debug)]
pub struct TankFarmMonthVentInterim {
    pub tank_farm_id: Uuid,
    pub month: NaiveDate,
    pub gas_volume: f64,
    pub c1_volume: f64,
    pub co2_volume: f64,
}

#[derive(Debug)]
pub struct TankFarmMonthVentInterimUnnestedRows {
    pub user_id: Uuid,
    pub tank_farm_month_vents_interim: Vec<TankFarmMonthVentInterim>,
}

#[derive(Debug)]
pub struct TankFarmMonthVentInterimNestedRows {
    pub id: Vec<Uuid>,
    pub tank_farm_id: Vec<Uuid>,
    pub month: Vec<NaiveDate>,
    pub gas_volume: Vec<f64>,
    pub c1_volume: Vec<f64>,
    pub co2_volume: Vec<f64>,
    pub created_by_id: Vec<Uuid>,
    pub created_at: Vec<NaiveDateTime>,
    pub updated_by_id: Vec<Uuid>,
    pub updated_at: Vec<NaiveDateTime>,
}

impl From<TankFarmMonthVentInterimUnnestedRows> for TankFarmMonthVentInterimNestedRows {
    fn from(
        TankFarmMonthVentInterimUnnestedRows {
            user_id,
            tank_farm_month_vents_interim,
        }: TankFarmMonthVentInterimUnnestedRows,
    ) -> Self {
        let (
            id,
            tank_farm_id,
            month,
            gas_volume,
            c1_volume,
            co2_volume,
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
        ) = tank_farm_month_vents_interim
            .into_iter()
            .map(|tfmvi| {
                (
                    Uuid::new_v4(),
                    tfmvi.tank_farm_id,
                    tfmvi.month,
                    tfmvi.gas_volume,
                    tfmvi.c1_volume,
                    tfmvi.co2_volume,
                    user_id.clone(),
                    chrono::Utc::now().naive_utc(),
                    user_id.clone(),
                    chrono::Utc::now().naive_utc(),
                )
            })
            .multiunzip();

        TankFarmMonthVentInterimNestedRows {
            id,
            tank_farm_id,
            month,
            gas_volume,
            c1_volume,
            co2_volume,
            created_by_id,
            created_at,
            updated_by_id,
            updated_at,
        }
    }
}

#[derive(Debug, OneofObject)]
pub enum TankFarmMonthVentBy {
    TankFarmId(Uuid),
    Month(NaiveDate),
}
