use super::Tank;
use crate::graphql::{
    context::ContextExt,
    dataloaders::{defined_vent_gas::tank::TankLoader, user::UserLoader},
    models::user::User,
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct TankMonthOilFlow {
    pub id: Uuid,
    pub tank_id: Uuid,
    pub month: NaiveDate,
    pub oil: f64,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl TankMonthOilFlow {
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

    async fn tank(&self, ctx: &Context<'_>) -> Result<Option<Tank>, Error> {
        let loader = ctx.get_loader::<DataLoader<TankLoader>>();
        let tank = loader.load_one(self.tank_id).await;

        tank
    }
}
