use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        compressor::{
            CreatedCompressorBlowdownsLoader, CreatedCompressorMonthHoursLoader,
            CreatedCompressorSealMonthMethaneEmissionOverridesLoader,
            CreatedCompressorSealTestsLoader, CreatedCompressorSealsLoader,
            CreatedCompressorsLoader, UpdatedCompressorBlowdownsLoader,
            UpdatedCompressorMonthHoursLoader,
            UpdatedCompressorSealMonthMethaneEmissionOverridesLoader,
            UpdatedCompressorSealTestsLoader, UpdatedCompressorsLoader, UpdatedCompressorSealsLoader,
        },
        defined_vent_gas::tank::{
            CreatedTankChangesLoader, CreatedTankEmissionFactorsCalculatedLoader,
            CreatedTankMonthMethaneEmissionOverridesLoader, CreatedTankMonthOilFlowsLoader,
            CreatedTanksLoader, UpdatedTankChangesLoader,
            UpdatedTankEmissionFactorsCalculatedLoader,
            UpdatedTankMonthMethaneEmissionOverridesLoader, UpdatedTankMonthOilFlowsLoader,
            UpdatedTanksLoader,
        },
        facility::{CreatedFacilitiesLoader, UpdatedFacilitiesLoader},
        gas_analysis::{
            CreatedGasAnalysesLoader, CreatedGasAnalysisCalculatedParamsLoader,
            UpdatedGasAnalysesLoader, UpdatedGasAnalysisCalculatedParamsLoader,
        },
        month_methane_emission::{
            CreatedMonthMethaneEmissionsLoader, UpdatedMonthMethaneEmissionsLoader,
        },
        pneumatic_device::{
            CreatedDeviceManufacturersLoader, CreatedPneumaticDeviceChangesLoader,
            CreatedPneumaticDeviceMonthHoursLoader,
            CreatedPneumaticDeviceMonthMethaneEmissionOverridesLoader,
            CreatedPneumaticDevicesLoader, UpdatedDeviceManufacturersLoader,
            UpdatedPneumaticDeviceChangesLoader, UpdatedPneumaticDeviceMonthHoursLoader,
            UpdatedPneumaticDeviceMonthMethaneEmissionOverridesLoader,
            UpdatedPneumaticDevicesLoader,
        },
        site::{CreatedSitesLoader, UpdatedSitesLoader},
    },
    models::{
        compressor::{
            Compressor, CompressorBlowdown, CompressorMonthHours, CompressorSeal,
            CompressorSealMonthMethaneEmissionOverride, CompressorSealTest,
        },
        defined_vent_gas::tank::{
            Tank, TankChange, TankEmissionFactorCalculated, TankMonthMethaneEmissionOverride,
            TankMonthOilFlow,
        },
        facility::Facility,
        gas_analysis::{GasAnalysis, GasAnalysisCalculatedParam},
        month_methane_emission::MonthMethaneEmission,
        pneumatic_device::{
            DeviceManufacturer, PneumaticDevice, PneumaticDeviceChange, PneumaticDeviceMonthHours,
            PneumaticDeviceMonthMethaneEmissionOverride,
        },
        site::Site,
    },
};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, Error, InputObject, OneofObject, SimpleObject,
};
use common::Role;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, OneofObject)]
pub enum UserBy {
    Email(String),
    Id(Uuid),
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

    async fn created_sites(&self, ctx: &Context<'_>) -> Result<Vec<Site>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedSitesLoader>>();
        let sites = loader.load_one(self.id).await?;
        let result = sites.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_sites(&self, ctx: &Context<'_>) -> Result<Vec<Site>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedSitesLoader>>();
        let sites = loader.load_one(self.id).await?;
        let result = sites.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_pneumatic_devices(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticDevice>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedPneumaticDevicesLoader>>();
        let pneumatic_devices = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no created pneumatic devices
        let result = pneumatic_devices.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_pneumatic_devices(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticDevice>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedPneumaticDevicesLoader>>();
        let pneumatic_devices = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no updated pneumatic devices
        let result = pneumatic_devices.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_device_manufacturers(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<DeviceManufacturer>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedDeviceManufacturersLoader>>();
        let device_manufacturers = loader.load_one(self.id).await?;
        let result = device_manufacturers.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_device_manufacturers(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<DeviceManufacturer>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedDeviceManufacturersLoader>>();
        let device_manufacturers = loader.load_one(self.id).await?;
        let result = device_manufacturers.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_pneumatic_device_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticDeviceChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedPneumaticDeviceChangesLoader>>();
        let pneumatic_device_changes = loader.load_one(self.id).await?;
        let result = pneumatic_device_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_pneumatic_device_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticDeviceChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedPneumaticDeviceChangesLoader>>();
        let pneumatic_device_changes = loader.load_one(self.id).await?;
        let result = pneumatic_device_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_pneumatic_device_month_hours(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticDeviceMonthHours>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedPneumaticDeviceMonthHoursLoader>>();
        let pneumatic_device_month_hours = loader.load_one(self.id).await?;
        let result = pneumatic_device_month_hours.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_pneumatic_device_month_hours(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticDeviceMonthHours>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedPneumaticDeviceMonthHoursLoader>>();
        let pneumatic_device_month_hours = loader.load_one(self.id).await?;
        let result = pneumatic_device_month_hours.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_pneumatic_device_month_methane_emission_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticDeviceMonthMethaneEmissionOverride>, Error> {
        let loader = ctx
            .get_loader::<DataLoader<CreatedPneumaticDeviceMonthMethaneEmissionOverridesLoader>>();
        let pneumatic_device_month_methane_emission_overrides = loader.load_one(self.id).await?;
        let result = pneumatic_device_month_methane_emission_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_pneumatic_device_month_methane_emission_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticDeviceMonthMethaneEmissionOverride>, Error> {
        let loader = ctx
            .get_loader::<DataLoader<UpdatedPneumaticDeviceMonthMethaneEmissionOverridesLoader>>();
        let pneumatic_device_month_methane_emission_overrides = loader.load_one(self.id).await?;
        let result = pneumatic_device_month_methane_emission_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_month_methane_emissions(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<MonthMethaneEmission>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedMonthMethaneEmissionsLoader>>();
        let month_methane_emissions = loader.load_one(self.id).await?;
        let result = month_methane_emissions.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_month_methane_emissions(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<MonthMethaneEmission>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedMonthMethaneEmissionsLoader>>();
        let month_methane_emissions = loader.load_one(self.id).await?;
        let result = month_methane_emissions.unwrap_or(vec![]);

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

    async fn created_compressor_seals(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorSeal>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedCompressorSealsLoader>>();
        let compressor_seals = loader.load_one(self.id).await?;
        let result = compressor_seals.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_compressor_seals(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorSeal>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedCompressorSealsLoader>>();
        let compressor_seals = loader.load_one(self.id).await?;
        let result = compressor_seals.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_compressor_seal_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorSealTest>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedCompressorSealTestsLoader>>();
        let compressor_seal_tests = loader.load_one(self.id).await?;
        let result = compressor_seal_tests.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_compressor_seal_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorSealTest>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedCompressorSealTestsLoader>>();
        let compressor_seal_tests = loader.load_one(self.id).await?;
        let result = compressor_seal_tests.unwrap_or(vec![]);

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

    async fn created_compressor_month_vent_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorMonthVentOverride>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedCompressorMonthVentOverridesLoader>>();
        let compressor_month_vent_overrides = loader.load_one(self.id).await?;
        let result = compressor_month_vent_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_compressor_month_vent_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorMonthVentOverride>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedCompressorMonthVentOverridesLoader>>();
        let compressor_month_vent_overrides = loader.load_one(self.id).await?;
        let result = compressor_month_vent_overrides.unwrap_or(vec![]);

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
