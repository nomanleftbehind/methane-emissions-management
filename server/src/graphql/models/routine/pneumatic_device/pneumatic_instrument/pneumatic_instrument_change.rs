use super::{super::super::super::user::User, PneumaticInstrument};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        routine::pneumatic_device::pneumatic_instrument::PneumaticInstrumentLoader,
        user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct PneumaticInstrumentChange {
    pub id: Uuid,
    pub pneumatic_instrument_id: Uuid,
    pub date: NaiveDate,
    /// standard cubic meters per hour of natural gas
    pub rate: f64,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl PneumaticInstrumentChange {
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

    async fn pneumatic_instrument(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<PneumaticInstrument>, Error> {
        let loader = ctx.get_loader::<DataLoader<PneumaticInstrumentLoader>>();
        let pneumatic_instrument = loader.load_one(self.pneumatic_instrument_id).await;

        pneumatic_instrument
    }
}
