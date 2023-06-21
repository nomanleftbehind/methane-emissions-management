use super::super::{
    facility::Facility,
    gas_analysis::{GasAnalysis, GasAnalysisCalculatedParam},
    month_methane_emission::MonthMethaneEmission,
    nonroutine::compressor_blowdown::{CompressorBlowdown, CompressorBlowdownOverride},
    routine::{
        compressor_seal::{
            Compressor, CompressorMonthHours, CompressorSeal,
            CompressorSealMonthMethaneEmissionOverride, CompressorSealTest,
        },
        defined_vent_gas::storage_tank::{
            StorageTank, StorageTankChange, StorageTankGasInSolutionFactorCalculated,
            StorageTankMonthLiquidHydrocarbonEntering, StorageTankMonthMethaneEmissionOverride,
        },
        pneumatic_device::{
            pneumatic_instrument::{
                PneumaticInstrument, PneumaticInstrumentChange, PneumaticInstrumentMonthHours,
                PneumaticInstrumentMonthMethaneEmissionOverride,
            },
            DeviceManufacturer,
        },
    },
    site::Site,
};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        facility::{CreatedFacilitiesLoader, UpdatedFacilitiesLoader},
        gas_analysis::{
            CreatedGasAnalysesLoader, CreatedGasAnalysisCalculatedParamsLoader,
            UpdatedGasAnalysesLoader, UpdatedGasAnalysisCalculatedParamsLoader,
        },
        month_methane_emission::{
            CreatedMonthMethaneEmissionsLoader, UpdatedMonthMethaneEmissionsLoader,
        },
        nonroutine::compressor_blowdown::{
            CreatedCompressorBlowdownOverridesLoader, CreatedCompressorBlowdownsLoader,
            UpdatedCompressorBlowdownOverridesLoader, UpdatedCompressorBlowdownsLoader,
        },
        routine::{
            compressor_seal::{
                CreatedCompressorMonthHoursLoader,
                CreatedCompressorSealMonthMethaneEmissionOverridesLoader,
                CreatedCompressorSealTestsLoader, CreatedCompressorSealsLoader,
                CreatedCompressorsLoader, UpdatedCompressorMonthHoursLoader,
                UpdatedCompressorSealMonthMethaneEmissionOverridesLoader,
                UpdatedCompressorSealTestsLoader, UpdatedCompressorSealsLoader,
                UpdatedCompressorsLoader,
            },
            defined_vent_gas::storage_tank::{
                CreatedStorageTankChangesLoader,
                CreatedStorageTankGasInSolutionFactorsCalculatedLoader,
                CreatedStorageTankMonthLiquidHydrocarbonEnteringLoader,
                CreatedStorageTankMonthMethaneEmissionOverridesLoader, CreatedStorageTanksLoader,
                UpdatedStorageTankChangesLoader,
                UpdatedStorageTankGasInSolutionFactorsCalculatedLoader,
                UpdatedStorageTankMonthLiquidHydrocarbonEnteringLoader,
                UpdatedStorageTankMonthMethaneEmissionOverridesLoader, UpdatedStorageTanksLoader,
            },
            pneumatic_device::{
                pneumatic_instrument::{
                    CreatedPneumaticInstrumentChangesLoader,
                    CreatedPneumaticInstrumentMonthHoursLoader,
                    CreatedPneumaticInstrumentMonthMethaneEmissionOverridesLoader,
                    CreatedPneumaticInstrumentsLoader, UpdatedPneumaticInstrumentChangesLoader,
                    UpdatedPneumaticInstrumentMonthHoursLoader,
                    UpdatedPneumaticInstrumentMonthMethaneEmissionOverridesLoader,
                    UpdatedPneumaticInstrumentsLoader,
                },
                CreatedDeviceManufacturersLoader, UpdatedDeviceManufacturersLoader,
            },
        },
        site::{CreatedSitesLoader, UpdatedSitesLoader},
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
    ) -> Result<Vec<PneumaticInstrument>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedPneumaticInstrumentsLoader>>();
        let pneumatic_devices = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no created pneumatic devices
        let result = pneumatic_devices.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_pneumatic_devices(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticInstrument>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedPneumaticInstrumentsLoader>>();
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
    ) -> Result<Vec<PneumaticInstrumentChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedPneumaticInstrumentChangesLoader>>();
        let pneumatic_device_changes = loader.load_one(self.id).await?;
        let result = pneumatic_device_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_pneumatic_device_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticInstrumentChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedPneumaticInstrumentChangesLoader>>();
        let pneumatic_device_changes = loader.load_one(self.id).await?;
        let result = pneumatic_device_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_pneumatic_device_month_hours(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticInstrumentMonthHours>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedPneumaticInstrumentMonthHoursLoader>>();
        let pneumatic_device_month_hours = loader.load_one(self.id).await?;
        let result = pneumatic_device_month_hours.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_pneumatic_device_month_hours(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticInstrumentMonthHours>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedPneumaticInstrumentMonthHoursLoader>>();
        let pneumatic_device_month_hours = loader.load_one(self.id).await?;
        let result = pneumatic_device_month_hours.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_pneumatic_device_month_methane_emission_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticInstrumentMonthMethaneEmissionOverride>, Error> {
        let loader = ctx
            .get_loader::<DataLoader<CreatedPneumaticInstrumentMonthMethaneEmissionOverridesLoader>>(
            );
        let pneumatic_device_month_methane_emission_overrides = loader.load_one(self.id).await?;
        let result = pneumatic_device_month_methane_emission_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_pneumatic_device_month_methane_emission_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PneumaticInstrumentMonthMethaneEmissionOverride>, Error> {
        let loader = ctx
            .get_loader::<DataLoader<UpdatedPneumaticInstrumentMonthMethaneEmissionOverridesLoader>>(
            );
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

    async fn created_compressor_blowdown_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorBlowdownOverride>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedCompressorBlowdownOverridesLoader>>();
        let compressor_blowdown_overrides = loader.load_one(self.id).await?;
        let result = compressor_blowdown_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_compressor_blowdown_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorBlowdownOverride>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedCompressorBlowdownOverridesLoader>>();
        let compressor_blowdown_overrides = loader.load_one(self.id).await?;
        let result = compressor_blowdown_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_compressor_seal_month_methane_emission_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorSealMonthMethaneEmissionOverride>, Error> {
        let loader = ctx
            .get_loader::<DataLoader<CreatedCompressorSealMonthMethaneEmissionOverridesLoader>>();
        let compressor_seal_month_methane_emission_overrides = loader.load_one(self.id).await?;
        let result = compressor_seal_month_methane_emission_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_compressor_seal_month_methane_emission_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorSealMonthMethaneEmissionOverride>, Error> {
        let loader = ctx
            .get_loader::<DataLoader<UpdatedCompressorSealMonthMethaneEmissionOverridesLoader>>();
        let compressor_seal_month_methane_emission_overrides = loader.load_one(self.id).await?;
        let result = compressor_seal_month_methane_emission_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_storage_tanks(&self, ctx: &Context<'_>) -> Result<Vec<StorageTank>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedStorageTanksLoader>>();
        let storage_tanks = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no created storage_tank farms
        let result = storage_tanks.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_storage_tanks(&self, ctx: &Context<'_>) -> Result<Vec<StorageTank>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedStorageTanksLoader>>();
        let storage_tanks = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no updated storage_tank farms
        let result = storage_tanks.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_storage_tank_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<StorageTankChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<CreatedStorageTankChangesLoader>>();
        let storage_tank_changes = loader.load_one(self.id).await?;
        let result = storage_tank_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_storage_tank_changes(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<StorageTankChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<UpdatedStorageTankChangesLoader>>();
        let storage_tank_changes = loader.load_one(self.id).await?;
        let result = storage_tank_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_storage_tank_month_liquid_hydrocarbon_entering(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<StorageTankMonthLiquidHydrocarbonEntering>, Error> {
        let loader =
            ctx.get_loader::<DataLoader<CreatedStorageTankMonthLiquidHydrocarbonEnteringLoader>>();
        let storage_tank_month_liquid_hydrocarbon_entering = loader.load_one(self.id).await?;
        let result = storage_tank_month_liquid_hydrocarbon_entering.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_storage_tank_month_liquid_hydrocarbon_entering(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<StorageTankMonthLiquidHydrocarbonEntering>, Error> {
        let loader =
            ctx.get_loader::<DataLoader<UpdatedStorageTankMonthLiquidHydrocarbonEnteringLoader>>();
        let storage_tank_month_liquid_hydrocarbon_entering = loader.load_one(self.id).await?;
        let result = storage_tank_month_liquid_hydrocarbon_entering.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_storage_tank_gas_in_solution_factors_calculated(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<StorageTankGasInSolutionFactorCalculated>, Error> {
        let loader =
            ctx.get_loader::<DataLoader<CreatedStorageTankGasInSolutionFactorsCalculatedLoader>>();
        let storage_tank_gas_in_solution_factors_calculated = loader.load_one(self.id).await?;
        let result = storage_tank_gas_in_solution_factors_calculated.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_storage_tank_gas_in_solution_factors_calculated(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<StorageTankGasInSolutionFactorCalculated>, Error> {
        let loader =
            ctx.get_loader::<DataLoader<UpdatedStorageTankGasInSolutionFactorsCalculatedLoader>>();
        let storage_tank_gas_in_solution_factors_calculated = loader.load_one(self.id).await?;
        let result = storage_tank_gas_in_solution_factors_calculated.unwrap_or(vec![]);

        Ok(result)
    }

    async fn created_storage_tank_month_methane_emission_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<StorageTankMonthMethaneEmissionOverride>, Error> {
        let loader =
            ctx.get_loader::<DataLoader<CreatedStorageTankMonthMethaneEmissionOverridesLoader>>();
        let storage_tank_month_methane_emission_overrides = loader.load_one(self.id).await?;
        let result = storage_tank_month_methane_emission_overrides.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_storage_tank_month_methane_emission_overrides(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<StorageTankMonthMethaneEmissionOverride>, Error> {
        let loader =
            ctx.get_loader::<DataLoader<UpdatedStorageTankMonthMethaneEmissionOverridesLoader>>();
        let storage_tank_month_methane_emission_overrides = loader.load_one(self.id).await?;
        let result = storage_tank_month_methane_emission_overrides.unwrap_or(vec![]);

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
