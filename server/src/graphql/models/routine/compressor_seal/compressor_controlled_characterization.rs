use super::{super::super::user::User, Compressor, CompressorControlDeviceInactivity};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        routine::compressor_seal::{
            CompressorControlDeviceInactivitiesByCompressorControlledCharacterizationLoader,
            CompressorLoader,
        },
        user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use common::ControlDevice;
use sqlx::FromRow;
use uuid::Uuid;

/// Object representing changes to controlled or uncontrolled characterization of compressor as defined in AER Directive 060 [`Section 8.11`](https://static.aer.ca/prd/documents/directives/Directive060.pdf#page=88).
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct CompressorControlledCharacterization {
    pub id: Uuid,
    pub compressor_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub control_device: ControlDevice,
    pub comment: Option<String>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl CompressorControlledCharacterization {
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

    async fn compressor_control_device_inactivities(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorControlDeviceInactivity>, Error> {
        let loader = ctx.get_loader::<DataLoader<
            CompressorControlDeviceInactivitiesByCompressorControlledCharacterizationLoader,
        >>();
        let compressor_control_device_inactivities = loader.load_one(self.id).await?;
        let result = compressor_control_device_inactivities.unwrap_or(vec![]);

        Ok(result)
    }
}
