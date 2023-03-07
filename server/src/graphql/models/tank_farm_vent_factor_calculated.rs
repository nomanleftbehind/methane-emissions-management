use crate::graphql::{
    context::ContextExt,
    dataloaders::{tank_farm_loader::TankFarmLoader, user_loader::UserLoader},
    models::{TankFarm, User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use itertools::MultiUnzip;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct TankFarmVentFactorCalculated {
    pub id: Uuid,
    pub tank_farm_id: Uuid,
    pub date: NaiveDate,
    pub vent_factor: f64,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl TankFarmVentFactorCalculated {
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
pub struct TankFarmVentFactorCalculatedInterim {
    pub tank_farm_id: Uuid,
    pub date: NaiveDate,
    pub vent_factor: f64,
}

#[derive(Debug)]
pub struct TankFarmVentFactorCalculatedInterimUnnestedRows {
    pub user_id: Uuid,
    pub tank_farm_vent_factors_calculated_interim: Vec<TankFarmVentFactorCalculatedInterim>,
}

#[derive(Debug)]
pub struct TankFarmVentFactorCalculatedInterimNestedRows {
    pub id: Vec<Uuid>,
    pub tank_farm_id: Vec<Uuid>,
    pub date: Vec<NaiveDate>,
    pub vent_factor: Vec<f64>,
    pub created_by_id: Vec<Uuid>,
    pub created_at: Vec<NaiveDateTime>,
    pub updated_by_id: Vec<Uuid>,
    pub updated_at: Vec<NaiveDateTime>,
}

impl From<TankFarmVentFactorCalculatedInterimUnnestedRows>
    for TankFarmVentFactorCalculatedInterimNestedRows
{
    fn from(
        TankFarmVentFactorCalculatedInterimUnnestedRows {
            user_id,
            tank_farm_vent_factors_calculated_interim,
        }: TankFarmVentFactorCalculatedInterimUnnestedRows,
    ) -> Self {
        let (
            id,
            tank_farm_id,
            date,
            vent_factor,
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
        ) = tank_farm_vent_factors_calculated_interim
            .into_iter()
            .map(|cmvc| {
                (
                    Uuid::new_v4(),
                    cmvc.tank_farm_id,
                    cmvc.date,
                    cmvc.vent_factor,
                    user_id.clone(),
                    chrono::Utc::now().naive_utc(),
                    user_id.clone(),
                    chrono::Utc::now().naive_utc(),
                )
            })
            .multiunzip();

        TankFarmVentFactorCalculatedInterimNestedRows {
            id,
            tank_farm_id,
            date,
            vent_factor,
            created_by_id,
            created_at,
            updated_by_id,
            updated_at,
        }
    }
}
