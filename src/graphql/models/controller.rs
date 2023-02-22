use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        controller_application_loader::ControllerApplicationLoader,
        controller_change_loader::ControllerChangesByControllerLoader,
        controller_manufacturer_loader::ControllerManufacturerLoader,
        controller_month_hours_loader::ControllerMonthHoursByControllerLoader,
        controller_month_vent_loader::ControllerMonthVentsByControllerLoader,
        facility_loader::FacilityLoader, user_loader::UserLoader,
    },
    models::{
        ControllerApplication, ControllerChange, ControllerManufacturer, ControllerMonthHours,
        ControllerMonthVent, Facility, User,
    },
};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, Error, OneofObject, SimpleObject,
};
use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct Controller {
    pub id: Uuid,
    pub fdc_rec_id: String,
    pub manufacturer_id: Uuid,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub application_id: Option<Uuid>,
    pub facility_id: Uuid,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Controller {
    pub(in crate::graphql) async fn created_by(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<User>, Error> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let created_by = loader.load_one(self.created_by_id).await;

        created_by
    }

    pub(in crate::graphql) async fn updated_by(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<User>, Error> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let updated_by = loader.load_one(self.updated_by_id).await;

        updated_by
    }

    pub(in crate::graphql) async fn facility(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<Facility>, Error> {
        let loader = ctx.get_loader::<DataLoader<FacilityLoader>>();
        let facility = loader.load_one(self.facility_id).await;

        facility
    }

    async fn application(&self, ctx: &Context<'_>) -> Result<Option<ControllerApplication>, Error> {
        let loader = ctx.get_loader::<DataLoader<ControllerApplicationLoader>>();
        let application = if let Some(id) = self.application_id {
            loader.load_one(id).await
        } else {
            Ok(None)
        };

        application
    }

    async fn manufacturer(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<ControllerManufacturer>, Error> {
        let loader = ctx.get_loader::<DataLoader<ControllerManufacturerLoader>>();
        let manufacturer = loader.load_one(self.manufacturer_id).await;

        manufacturer
    }

    async fn controller_changes(&self, ctx: &Context<'_>) -> Result<Vec<ControllerChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<ControllerChangesByControllerLoader>>();
        let controller_changes = loader.load_one(self.id).await?;
        let result = controller_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn controller_month_hours(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerMonthHours>, Error> {
        let loader = ctx.get_loader::<DataLoader<ControllerMonthHoursByControllerLoader>>();
        let controller_month_hours = loader.load_one(self.id).await?;
        let result = controller_month_hours.unwrap_or(vec![]);

        Ok(result)
    }

    async fn controller_month_vents(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerMonthVent>, Error> {
        let loader = ctx.get_loader::<DataLoader<ControllerMonthVentsByControllerLoader>>();
        let controller_month_vents = loader.load_one(self.id).await?;
        let result = controller_month_vents.unwrap_or(vec![]);

        Ok(result)
    }
}

#[derive(Debug, OneofObject)]
pub enum ControllersBy {
    FacilityId(Uuid),
    CreatedById(Uuid),
    UpdatedById(Uuid),
}
