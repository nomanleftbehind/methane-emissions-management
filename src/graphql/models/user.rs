use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        compressor_blowdown_loader::{
            CreatedCompressorBlowdownsLoader, UpdatedCompressorBlowdownsLoader,
        },
        compressor_change_loader::{
            CreatedCompressorChangesLoader, UpdatedCompressorChangesLoader,
        },
        compressor_loader::{CreatedCompressorsLoader, UpdatedCompressorsLoader},
        compressor_month_hours_loader::{
            CreatedCompressorMonthHoursLoader, UpdatedCompressorMonthHoursLoader,
        },
        compressor_month_vent_loader::{
            CreatedCompressorMonthVentsLoader, UpdatedCompressorMonthVentsLoader,
        },
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
        controller_month_hours_loader::{
            CreatedControllerMonthHoursLoader, UpdatedControllerMonthHoursLoader,
        },
        controller_month_vent_loader::{
            CreatedControllerMonthVentsLoader, UpdatedControllerMonthVentsLoader,
        },
        facility_loader::{CreatedFacilitiesLoader, UpdatedFacilitiesLoader},
        gas_analysis_calculated_param_loader::{
            CreatedGasAnalysisCalculatedParamsLoader, UpdatedGasAnalysisCalculatedParamsLoader,
        },
        gas_analysis_loader::{CreatedGasAnalysesLoader, UpdatedGasAnalysesLoader},
        tank_farm_change_loader::{CreatedTankFarmChangesLoader, UpdatedTankFarmChangesLoader},
        tank_farm_loader::{CreatedTankFarmsLoader, UpdatedTankFarmsLoader},
        tank_farm_month_oil_flow_loader::{
            CreatedTankFarmMonthOilFlowsLoader, UpdatedTankFarmMonthOilFlowsLoader,
        },
        tank_farm_month_vent_loader::{
            CreatedTankFarmMonthVentsLoader, UpdatedTankFarmMonthVentsLoader,
        },
        tank_farm_month_vent_override_loader::{
            CreatedTankFarmMonthVentOverridesLoader, UpdatedTankFarmMonthVentOverridesLoader,
        },
        tank_farm_vent_factor_loader::{
            CreatedTankFarmVentFactorsCalculatedLoader, UpdatedTankFarmVentFactorsCalculatedLoader,
        },
    },
    models::{
        Compressor, CompressorBlowdown, CompressorChange, CompressorMonthHours,
        CompressorMonthVent, Controller, ControllerApplication, ControllerChange,
        ControllerManufacturer, ControllerMonthHours, ControllerMonthVent, Facility, GasAnalysis,
        GasAnalysisCalculatedParam, TankFarm, TankFarmChange, TankFarmMonthOilFlow,
        TankFarmMonthVent, TankFarmMonthVentOverride, TankFarmVentFactorCalculated,
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

/// `User` model is the root of every other model.
///
/// Every entry was created and updated by a `User` so the model can load an array of created or updated entries.
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
        // Need to return empty vector if user has no created controllers
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

    async fn created_controller_month_hours(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerMonthHours>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedControllerMonthHoursLoader>>();
        let controller_month_hours = loader.load_one(self.id).await?;
        let result = controller_month_hours.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_controller_month_hours(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerMonthHours>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedControllerMonthHoursLoader>>();
        let controller_month_hours = loader.load_one(self.id).await?;
        let result = controller_month_hours.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_controller_month_vents(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerMonthVent>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedControllerMonthVentsLoader>>();
        let controller_month_vents = loader.load_one(self.id).await?;
        let result = controller_month_vents.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_controller_month_vents(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerMonthVent>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedControllerMonthVentsLoader>>();
        let controller_month_vents = loader.load_one(self.id).await?;
        let result = controller_month_vents.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_compressors(&self, ctx: &Context<'_>) -> Result<Vec<Compressor>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedCompressorsLoader>>();
        let compressors = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no created compressors
        let result = compressors.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_compressors(&self, ctx: &Context<'_>) -> Result<Vec<Compressor>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedCompressorsLoader>>();
        let compressors = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no updated compressors
        let result = compressors.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_compressor_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedCompressorChangesLoader>>();
        let compressor_changes = loader.load_one(self.id).await?;
        let result = compressor_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_compressor_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedCompressorChangesLoader>>();
        let compressor_changes = loader.load_one(self.id).await?;
        let result = compressor_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_compressor_month_hours(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorMonthHours>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedCompressorMonthHoursLoader>>();
        let compressor_month_hours = loader.load_one(self.id).await?;
        let result = compressor_month_hours.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_compressor_month_hours(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorMonthHours>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedCompressorMonthHoursLoader>>();
        let compressor_month_hours = loader.load_one(self.id).await?;
        let result = compressor_month_hours.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_compressor_blowdowns(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorBlowdown>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedCompressorBlowdownsLoader>>();
        let compressor_blowdowns = loader.load_one(self.id).await?;
        let result = compressor_blowdowns.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_compressor_blowdowns(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorBlowdown>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedCompressorBlowdownsLoader>>();
        let compressor_blowdowns = loader.load_one(self.id).await?;
        let result = compressor_blowdowns.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_compressor_month_vents(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorMonthVent>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedCompressorMonthVentsLoader>>();
        let compressor_month_vents = loader.load_one(self.id).await?;
        let result = compressor_month_vents.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_compressor_month_vents(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorMonthVent>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedCompressorMonthVentsLoader>>();
        let compressor_month_vents = loader.load_one(self.id).await?;
        let result = compressor_month_vents.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_tank_farms(&self, ctx: &Context<'_>) -> Result<Vec<TankFarm>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedTankFarmsLoader>>();
        let tank_farms = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no created tank farms
        let result = tank_farms.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_tank_farms(&self, ctx: &Context<'_>) -> Result<Vec<TankFarm>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedTankFarmsLoader>>();
        let tank_farms = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no updated tank farms
        let result = tank_farms.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_tank_farm_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TankFarmChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedTankFarmChangesLoader>>();
        let tank_farm_changes = loader.load_one(self.id).await?;
        let result = tank_farm_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_tank_farm_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TankFarmChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedTankFarmChangesLoader>>();
        let tank_farm_changes = loader.load_one(self.id).await?;
        let result = tank_farm_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_tank_farm_month_oil_flows(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TankFarmMonthOilFlow>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedTankFarmMonthOilFlowsLoader>>();
        let tank_farm_month_oil_flows = loader.load_one(self.id).await?;
        let result = tank_farm_month_oil_flows.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_tank_farm_month_oil_flows(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TankFarmMonthOilFlow>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedTankFarmMonthOilFlowsLoader>>();
        let tank_farm_month_oil_flows = loader.load_one(self.id).await?;
        let result = tank_farm_month_oil_flows.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_tank_farm_vent_factors_calculated(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TankFarmVentFactorCalculated>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedTankFarmVentFactorsCalculatedLoader>>();
        let tank_farm_vent_factors_calculated = loader.load_one(self.id).await?;
        let result = tank_farm_vent_factors_calculated.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_tank_farm_vent_factors_calculated(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TankFarmVentFactorCalculated>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedTankFarmVentFactorsCalculatedLoader>>();
        let tank_farm_vent_factors_calculated = loader.load_one(self.id).await?;
        let result = tank_farm_vent_factors_calculated.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_tank_farm_month_vent_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TankFarmMonthVentOverride>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedTankFarmMonthVentOverridesLoader>>();
        let tank_farm_month_vent_overrides = loader.load_one(self.id).await?;
        let result = tank_farm_month_vent_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_tank_farm_month_vent_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TankFarmMonthVentOverride>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedTankFarmMonthVentOverridesLoader>>();
        let tank_farm_month_vent_overrides = loader.load_one(self.id).await?;
        let result = tank_farm_month_vent_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_tank_farm_month_vents(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TankFarmMonthVent>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedTankFarmMonthVentsLoader>>();
        let tank_farm_month_vents = loader.load_one(self.id).await?;
        let result = tank_farm_month_vents.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_tank_farm_month_vents(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TankFarmMonthVent>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedTankFarmMonthVentsLoader>>();
        let tank_farm_month_vents = loader.load_one(self.id).await?;
        let result = tank_farm_month_vents.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_gas_analyses(&self, ctx: &Context<'_>) -> Result<Vec<GasAnalysis>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedGasAnalysesLoader>>();
        let gas_analyses = loader.load_one(self.id).await?;
        let result = gas_analyses.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_gas_analyses(&self, ctx: &Context<'_>) -> Result<Vec<GasAnalysis>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedGasAnalysesLoader>>();
        let gas_analyses = loader.load_one(self.id).await?;
        let result = gas_analyses.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_gas_analysis_calculated_params(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<GasAnalysisCalculatedParam>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedGasAnalysisCalculatedParamsLoader>>();
        let gas_analysis_calculated_params = loader.load_one(self.id).await?;
        let result = gas_analysis_calculated_params.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_gas_analysis_calculated_params(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<GasAnalysisCalculatedParam>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedGasAnalysisCalculatedParamsLoader>>();
        let gas_analysis_calculated_params = loader.load_one(self.id).await?;
        let result = gas_analysis_calculated_params.unwrap_or(vec![]);

        Ok(result)
    }
}

#[derive(InputObject, Debug)]
pub struct RegisterUserInput {
    #[graphql(validator(email))]
    pub email: String,
    #[graphql(secret, validator(min_length = 8))]
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: Option<Role>,
}

#[derive(InputObject, Debug)]
pub struct LoginUserInput {
    #[graphql(validator(email))]
    pub email: String,
    #[graphql(secret)]
    pub password: String,
}
