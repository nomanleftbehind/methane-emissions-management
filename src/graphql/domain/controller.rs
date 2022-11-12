use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        controller_change_loader::ControllerChangesByControllerLoader,
        controller_function_loader::ControllerFunctionLoader,
        controller_manufacturer_loader::ControllerManufacturerLoader,
        facility_loader::FacilityLoader, user_loader::UserLoader,
    },
    domain::{ControllerChange, ControllerFunction, ControllerManufacturer, Facility, User},
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
    pub function_id: Option<Uuid>,
    pub facility_id: Uuid,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Controller {
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

    async fn facility(&self, ctx: &Context<'_>) -> Result<Option<Facility>, Error> {
        let loader = ctx.get_loader::<DataLoader<FacilityLoader>>();
        let facility = loader.load_one(self.facility_id).await;

        facility
    }

    async fn function(&self, ctx: &Context<'_>) -> Result<Option<ControllerFunction>, Error> {
        let loader = ctx.get_loader::<DataLoader<ControllerFunctionLoader>>();
        let function = if let Some(id) = self.function_id {
            loader.load_one(id).await
        } else {
            Ok(None)
        };

        function
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
        let controllers = loader.load_one(self.id).await?;
        let result = controllers.unwrap_or(vec![]);

        Ok(result)
    }
}

#[derive(Debug, OneofObject)]
pub enum ControllersBy {
    FacilityId(Uuid),
    CreatedById(Uuid),
    UpdatedById(Uuid),
}
