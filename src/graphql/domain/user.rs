use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        controller_application_loader::{
            CreatedControllerApplicationsLoader, UpdatedControllerApplicationsLoader,
        },
        controller_change_loader::{
            CreatedControllerChangesLoader, UpdatedControllerChangesLoader,
        },
        controller_loader::{CreatedControllersLoader, UpdatedControllersLoader},
        controller_manufacturer_loader::{
            CreatedControllerManufacturersLoader, UpdatedControllerManufacturersLoader,
        },
        facility_loader::{CreatedFacilitiesLoader, UpdatedFacilitiesLoader},
    },
    domain::{
        Controller, ControllerApplication, ControllerChange, ControllerManufacturer, Facility,
    },
};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, Enum, Error, InputObject, OneofObject,
    SimpleObject,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, OneofObject)]
pub enum UserBy {
    Email(String),
    Id(Uuid),
}

#[derive(Enum, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Debug, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "UPPERCASE")]
pub enum Role {
    Admin,
    Engineer,
    Regulatory,
    Office,
    Operator,
}

#[derive(Serialize, Deserialize, SimpleObject, Debug, Clone, FromRow)]
#[graphql(complex)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[graphql(skip)]
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: Role,
}

#[ComplexObject]
impl User {
    async fn created_facilities(&self, ctx: &Context<'_>) -> Result<Vec<Facility>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedFacilitiesLoader>>();
        let facilities = loader.load_one(self.id).await?;
        let result = facilities.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_facilities(&self, ctx: &Context<'_>) -> Result<Vec<Facility>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedFacilitiesLoader>>();
        let facilities = loader.load_one(self.id).await?;
        let result = facilities.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_controllers(&self, ctx: &Context<'_>) -> Result<Vec<Controller>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedControllersLoader>>();
        let controllers = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no written controllers
        let result = controllers.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_controllers(&self, ctx: &Context<'_>) -> Result<Vec<Controller>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedControllersLoader>>();
        let controllers = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no updated controllers
        let result = controllers.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_controller_applications(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerApplication>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedControllerApplicationsLoader>>();
        let controller_applications = loader.load_one(self.id).await?;
        let result = controller_applications.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_controller_applications(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerApplication>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedControllerApplicationsLoader>>();
        let controller_applications = loader.load_one(self.id).await?;
        let result = controller_applications.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_controller_manufacturers(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerManufacturer>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedControllerManufacturersLoader>>();
        let controller_manufacturers = loader.load_one(self.id).await?;
        let result = controller_manufacturers.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_controller_manufacturers(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerManufacturer>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedControllerManufacturersLoader>>();
        let controller_manufacturers = loader.load_one(self.id).await?;
        let result = controller_manufacturers.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_controller_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedControllerChangesLoader>>();
        let controller_changes = loader.load_one(self.id).await?;
        let result = controller_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_controller_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedControllerChangesLoader>>();
        let controller_changes = loader.load_one(self.id).await?;
        let result = controller_changes.unwrap_or(vec![]);

        Ok(result)
    }
}

#[derive(InputObject, Debug)]
pub struct RegisterUserInput {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: Option<Role>,
}

#[derive(InputObject, Debug)]
pub struct LoginUserInput {
    pub email: String,
    pub password: String,
}
