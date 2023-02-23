use crate::graphql::{
    context::ContextExt,
    dataloaders::{tank_farm_loader::TankFarmLoader, user_loader::UserLoader},
    models::{TankFarm, User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
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
