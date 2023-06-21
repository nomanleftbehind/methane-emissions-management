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

/// Model representing user overrides of calculated monthly methane emission volumes from non-level controllers.
///
/// Field `month` is a [`NaiveDate`](chrono::NaiveDate), which must be first day of the month. This is impossible to enforce on database level, but is instead guaranteed through [`MonthBeginningValidator`](crate::graphql::models::validators::MonthBeginningValidator).
///
/// Field `gas_volume` is in m³.
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct PneumaticInstrumentMonthMethaneEmissionOverride {
    pub id: Uuid,
    pub pneumatic_instrument_id: Uuid,
    pub month: NaiveDate,
    pub gas_volume: f64,
    pub comment: Option<String>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl PneumaticInstrumentMonthMethaneEmissionOverride {
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
