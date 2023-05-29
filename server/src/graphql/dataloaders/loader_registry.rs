use super::{
    facility::{CreatedFacilitiesLoader, FacilityLoader, UpdatedFacilitiesLoader},
    gas_analysis::{
        CreatedGasAnalysesLoader, CreatedGasAnalysisCalculatedParamsLoader,
        GasAnalysesByFacilityLoader, GasAnalysisCalculatedParamLoader, GasAnalysisLoader,
        UpdatedGasAnalysesLoader, UpdatedGasAnalysisCalculatedParamsLoader,
    },
    month_methane_emission::{
        CreatedMonthMethaneEmissionsLoader, MonthMethaneEmissionLoader,
        MonthMethaneEmissionsByFacilityLoader, MonthMethaneEmissionsBySiteLoader,
        MonthMethaneEmissionsBySourceTableLoader, UpdatedMonthMethaneEmissionsLoader,
    },
    nonroutine::compressor_blowdown::{
        CompressorBlowdownLoader, CompressorBlowdownOverrideLoader,
        CompressorBlowdownOverridesByCompressorLoader, CompressorBlowdownsByCompressorLoader,
        CreatedCompressorBlowdownOverridesLoader, CreatedCompressorBlowdownsLoader,
        UpdatedCompressorBlowdownOverridesLoader, UpdatedCompressorBlowdownsLoader,
    },
    routine::{
        compressor_seal::{
            CompressorControlledCharacterizationLoader,
            CompressorControlledCharacterizationsByCompressorLoader, CompressorLoader,
            CompressorMonthHoursByCompressorLoader, CompressorMonthHoursLoader,
            CompressorSealLoader, CompressorSealMonthMethaneEmissionOverrideLoader,
            CompressorSealMonthMethaneEmissionOverridesByCompressorSealLoader,
            CompressorSealTestLoader, CompressorSealTestsByCompressorSealLoader,
            CompressorSealTestsBySurveyEquipmentLoader,
            CreatedCompressorControlledCharacterizationsLoader, CreatedCompressorMonthHoursLoader,
            CreatedCompressorSealMonthMethaneEmissionOverridesLoader,
            CreatedCompressorSealTestsLoader, CreatedCompressorSealsLoader,
            CreatedCompressorsLoader, SiteCompressorsLoader,
            UpdatedCompressorControlledCharacterizationsLoader, UpdatedCompressorMonthHoursLoader,
            UpdatedCompressorSealMonthMethaneEmissionOverridesLoader,
            UpdatedCompressorSealTestsLoader, UpdatedCompressorSealsLoader,
            UpdatedCompressorsLoader,
        },
        defined_vent_gas::storage_tank::{
            CreatedStorageTankChangesLoader, CreatedStorageTankControlDeviceInactivitiesLoader,
            CreatedStorageTankControlledCharacterizationsLoader,
            CreatedStorageTankEmissionSurveysLoader,
            CreatedStorageTankGasInSolutionFactorsCalculatedLoader,
            CreatedStorageTankMonthLiquidHydrocarbonEnteringLoader,
            CreatedStorageTankMonthMethaneEmissionOverridesLoader, CreatedStorageTanksLoader,
            SiteStorageTanksLoader, StorageTankChangeLoader, StorageTankChangesByStorageTankLoader,
            StorageTankControlDeviceInactivitiesByStorageTankControlledCharacterizationLoader,
            StorageTankControlDeviceInactivityLoader, StorageTankControlledCharacterizationLoader,
            StorageTankControlledCharacterizationsByStorageTankLoader,
            StorageTankEmissionSurveyLoader, StorageTankEmissionSurveysByStorageTankLoader,
            StorageTankEmissionSurveysBySurveyEquipmentLoader,
            StorageTankGasInSolutionFactorCalculatedLoader,
            StorageTankGasInSolutionFactorsCalculatedByStorageTankLoader, StorageTankLoader,
            StorageTankMonthLiquidHydrocarbonEnteringByStorageTankLoader,
            StorageTankMonthLiquidHydrocarbonEnteringLoader,
            StorageTankMonthMethaneEmissionOverrideLoader,
            StorageTankMonthMethaneEmissionOverridesByStorageTankLoader,
            UpdatedStorageTankChangesLoader, UpdatedStorageTankControlDeviceInactivitiesLoader,
            UpdatedStorageTankControlledCharacterizationsLoader,
            UpdatedStorageTankEmissionSurveysLoader,
            UpdatedStorageTankGasInSolutionFactorsCalculatedLoader,
            UpdatedStorageTankMonthLiquidHydrocarbonEnteringLoader,
            UpdatedStorageTankMonthMethaneEmissionOverridesLoader, UpdatedStorageTanksLoader,
        },
        pneumatic_device::{
            level_controller::{
                CreatedLevelControllerActuationFrequenciesLoader,
                CreatedLevelControllerChangesLoader, CreatedLevelControllerMonthHoursLoader,
                CreatedLevelControllerMonthMethaneEmissionOverridesLoader,
                CreatedLevelControllersLoader,
                LevelControllerActuationFrequenciesByLevelControllerLoader,
                LevelControllerActuationFrequencyLoader, LevelControllerChangeLoader,
                LevelControllerChangesByLevelControllerLoader, LevelControllerLoader,
                LevelControllerMonthHoursByLevelControllerLoader, LevelControllerMonthHoursLoader,
                LevelControllerMonthMethaneEmissionOverrideLoader,
                LevelControllerMonthMethaneEmissionOverridesByLevelControllerLoader,
                LevelControllersByManufacturerLoader, SiteLevelControllersLoader,
                UpdatedLevelControllerActuationFrequenciesLoader,
                UpdatedLevelControllerChangesLoader, UpdatedLevelControllerMonthHoursLoader,
                UpdatedLevelControllerMonthMethaneEmissionOverridesLoader,
                UpdatedLevelControllersLoader,
            },
            non_level_controller::{
                CreatedNonLevelControllerChangesLoader, CreatedNonLevelControllerMonthHoursLoader,
                CreatedNonLevelControllerMonthMethaneEmissionOverridesLoader,
                CreatedNonLevelControllersLoader, NonLevelControllerChangeLoader,
                NonLevelControllerChangesByNonLevelControllerLoader, NonLevelControllerLoader,
                NonLevelControllerMonthHoursByNonLevelControllerLoader,
                NonLevelControllerMonthHoursLoader,
                NonLevelControllerMonthMethaneEmissionOverrideLoader,
                NonLevelControllerMonthMethaneEmissionOverridesByNonLevelControllerLoader,
                NonLevelControllersByManufacturerLoader, SiteNonLevelControllersLoader,
                UpdatedNonLevelControllerChangesLoader, UpdatedNonLevelControllerMonthHoursLoader,
                UpdatedNonLevelControllerMonthMethaneEmissionOverridesLoader,
                UpdatedNonLevelControllersLoader,
            },
            CreatedDeviceManufacturersLoader, DeviceManufacturerLoader,
            UpdatedDeviceManufacturersLoader,
        },
    },
    site::{CreatedSitesLoader, FacilitySitesLoader, SiteLoader, UpdatedSitesLoader},
    survey_equipment::{
        CreatedSurveyEquipmentLoader, SurveyEquipmentLoader, UpdatedSurveyEquipmentLoader,
    },
    user::UserLoader,
};
use actix_web::web::Data;
use anymap::{any::Any, Map};
use async_graphql::dataloader::DataLoader;
use sqlx::PgPool;

pub type LoaderMap = Map<AnyLoader>;
pub type AnyLoader = dyn Any + Send + Sync;

pub struct LoaderRegistry {
    pub loaders: LoaderMap,
}

impl LoaderRegistry {
    pub fn get<T: anymap::any::Any + Send + Sync>(&self) -> &T {
        match self.loaders.get::<T>() {
            Some(loader) => loader,
            None => unreachable!("{} not found", std::any::type_name::<T>()),
        }
    }
}

pub async fn get_loaders(pool: Data<PgPool>) -> LoaderMap {
    let mut loaders: LoaderMap = LoaderMap::new();

    // User
    let user_by_id_loader = DataLoader::new(UserLoader::new(pool.clone()), tokio::spawn);

    // Facility
    let facility_by_id_loader = DataLoader::new(FacilityLoader::new(pool.clone()), tokio::spawn);
    let facilities_by_creator_id_loader =
        DataLoader::new(CreatedFacilitiesLoader::new(pool.clone()), tokio::spawn);
    let facilities_by_updater_id_loader =
        DataLoader::new(UpdatedFacilitiesLoader::new(pool.clone()), tokio::spawn);

    // Site
    let site_by_id_loader = DataLoader::new(SiteLoader::new(pool.clone()), tokio::spawn);
    let sites_by_facility_id_loader =
        DataLoader::new(FacilitySitesLoader::new(pool.clone()), tokio::spawn);
    let sites_by_creator_id_loader =
        DataLoader::new(CreatedSitesLoader::new(pool.clone()), tokio::spawn);
    let sites_by_updater_id_loader =
        DataLoader::new(UpdatedSitesLoader::new(pool.clone()), tokio::spawn);

    // Survey Equipment
    let survey_equipment_by_id_loader =
        DataLoader::new(SurveyEquipmentLoader::new(pool.clone()), tokio::spawn);
    let survey_equipment_by_creator_id_loader = DataLoader::new(
        CreatedSurveyEquipmentLoader::new(pool.clone()),
        tokio::spawn,
    );
    let survey_equipment_by_updater_id_loader = DataLoader::new(
        UpdatedSurveyEquipmentLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Non-Level Controller
    let non_level_controller_by_id_loader =
        DataLoader::new(NonLevelControllerLoader::new(pool.clone()), tokio::spawn);
    let non_level_controllers_by_site_id_loader = DataLoader::new(
        SiteNonLevelControllersLoader::new(pool.clone()),
        tokio::spawn,
    );
    let non_level_controllers_by_manufacturer_id_loader = DataLoader::new(
        NonLevelControllersByManufacturerLoader::new(pool.clone()),
        tokio::spawn,
    );
    let non_level_controllers_by_creator_id_loader = DataLoader::new(
        CreatedNonLevelControllersLoader::new(pool.clone()),
        tokio::spawn,
    );
    let non_level_controllers_by_updater_id_loader = DataLoader::new(
        UpdatedNonLevelControllersLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Device Manufacturer
    let device_manufacturer_by_id_loader =
        DataLoader::new(DeviceManufacturerLoader::new(pool.clone()), tokio::spawn);
    let device_manufacturers_by_creator_id_loader = DataLoader::new(
        CreatedDeviceManufacturersLoader::new(pool.clone()),
        tokio::spawn,
    );
    let device_manufacturers_by_updater_id_loader = DataLoader::new(
        UpdatedDeviceManufacturersLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Non-Level Controller Change
    let non_level_controller_change_by_id_loader = DataLoader::new(
        NonLevelControllerChangeLoader::new(pool.clone()),
        tokio::spawn,
    );
    let non_level_controller_changes_by_non_level_controller_id_loader = DataLoader::new(
        NonLevelControllerChangesByNonLevelControllerLoader::new(pool.clone()),
        tokio::spawn,
    );
    let non_level_controller_changes_by_creator_id_loader = DataLoader::new(
        CreatedNonLevelControllerChangesLoader::new(pool.clone()),
        tokio::spawn,
    );
    let non_level_controller_changes_by_updater_id_loader = DataLoader::new(
        UpdatedNonLevelControllerChangesLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Non-Level Controller Month Hours
    let non_level_controller_month_hours_by_id_loader = DataLoader::new(
        NonLevelControllerMonthHoursLoader::new(pool.clone()),
        tokio::spawn,
    );
    let non_level_controller_month_hours_by_non_level_controller_id_loader = DataLoader::new(
        NonLevelControllerMonthHoursByNonLevelControllerLoader::new(pool.clone()),
        tokio::spawn,
    );
    let non_level_controller_month_hours_by_creator_id_loader = DataLoader::new(
        CreatedNonLevelControllerMonthHoursLoader::new(pool.clone()),
        tokio::spawn,
    );
    let non_level_controller_month_hours_by_updater_id_loader = DataLoader::new(
        UpdatedNonLevelControllerMonthHoursLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Non-Level Controller Month Methane Emission Override
    let non_level_controller_month_methane_emission_override_by_id_loader = DataLoader::new(
        NonLevelControllerMonthMethaneEmissionOverrideLoader::new(pool.clone()),
        tokio::spawn,
    );
    let non_level_controller_month_methane_emission_overrides_by_non_level_controller_id_loader =
        DataLoader::new(
            NonLevelControllerMonthMethaneEmissionOverridesByNonLevelControllerLoader::new(
                pool.clone(),
            ),
            tokio::spawn,
        );
    let non_level_controller_month_methane_emission_overrides_by_creator_id_loader =
        DataLoader::new(
            CreatedNonLevelControllerMonthMethaneEmissionOverridesLoader::new(pool.clone()),
            tokio::spawn,
        );
    let non_level_controller_month_methane_emission_overrides_by_updater_id_loader =
        DataLoader::new(
            UpdatedNonLevelControllerMonthMethaneEmissionOverridesLoader::new(pool.clone()),
            tokio::spawn,
        );

    // Level Controller
    let level_controller_by_id_loader =
        DataLoader::new(LevelControllerLoader::new(pool.clone()), tokio::spawn);
    let level_controllers_by_site_id_loader =
        DataLoader::new(SiteLevelControllersLoader::new(pool.clone()), tokio::spawn);
    let level_controllers_by_manufacturer_id_loader = DataLoader::new(
        LevelControllersByManufacturerLoader::new(pool.clone()),
        tokio::spawn,
    );
    let level_controllers_by_creator_id_loader = DataLoader::new(
        CreatedLevelControllersLoader::new(pool.clone()),
        tokio::spawn,
    );
    let level_controllers_by_updater_id_loader = DataLoader::new(
        UpdatedLevelControllersLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Level Controller Actuation Frequency
    let level_controller_actuation_frequency_by_id_loader = DataLoader::new(
        LevelControllerActuationFrequencyLoader::new(pool.clone()),
        tokio::spawn,
    );
    let level_controller_actuation_frequencies_by_level_controller_id_loader = DataLoader::new(
        LevelControllerActuationFrequenciesByLevelControllerLoader::new(pool.clone()),
        tokio::spawn,
    );
    let level_controller_actuation_frequencies_by_creator_id_loader = DataLoader::new(
        CreatedLevelControllerActuationFrequenciesLoader::new(pool.clone()),
        tokio::spawn,
    );
    let level_controller_actuation_frequencies_by_updater_id_loader = DataLoader::new(
        UpdatedLevelControllerActuationFrequenciesLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Level Controller Change
    let level_controller_change_by_id_loader =
        DataLoader::new(LevelControllerChangeLoader::new(pool.clone()), tokio::spawn);
    let level_controller_changes_by_level_controller_id_loader = DataLoader::new(
        LevelControllerChangesByLevelControllerLoader::new(pool.clone()),
        tokio::spawn,
    );
    let level_controller_changes_by_creator_id_loader = DataLoader::new(
        CreatedLevelControllerChangesLoader::new(pool.clone()),
        tokio::spawn,
    );
    let level_controller_changes_by_updater_id_loader = DataLoader::new(
        UpdatedLevelControllerChangesLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Level Controller Month Hours
    let level_controller_month_hours_by_id_loader = DataLoader::new(
        LevelControllerMonthHoursLoader::new(pool.clone()),
        tokio::spawn,
    );
    let level_controller_month_hours_by_level_controller_id_loader = DataLoader::new(
        LevelControllerMonthHoursByLevelControllerLoader::new(pool.clone()),
        tokio::spawn,
    );
    let level_controller_month_hours_by_creator_id_loader = DataLoader::new(
        CreatedLevelControllerMonthHoursLoader::new(pool.clone()),
        tokio::spawn,
    );
    let level_controller_month_hours_by_updater_id_loader = DataLoader::new(
        UpdatedLevelControllerMonthHoursLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Level Controller Month Methane Emission Override
    let level_controller_month_methane_emission_override_by_id_loader = DataLoader::new(
        LevelControllerMonthMethaneEmissionOverrideLoader::new(pool.clone()),
        tokio::spawn,
    );
    let level_controller_month_methane_emission_overrides_by_level_controller_id_loader =
        DataLoader::new(
            LevelControllerMonthMethaneEmissionOverridesByLevelControllerLoader::new(pool.clone()),
            tokio::spawn,
        );
    let level_controller_month_methane_emission_overrides_by_creator_id_loader = DataLoader::new(
        CreatedLevelControllerMonthMethaneEmissionOverridesLoader::new(pool.clone()),
        tokio::spawn,
    );
    let level_controller_month_methane_emission_overrides_by_updater_id_loader = DataLoader::new(
        UpdatedLevelControllerMonthMethaneEmissionOverridesLoader::new(pool.clone()),
        tokio::spawn,
    );

    //  Month Methane Emission
    let month_methane_emission_by_id_loader =
        DataLoader::new(MonthMethaneEmissionLoader::new(pool.clone()), tokio::spawn);
    let month_methane_emissions_by_facility_id_loader = DataLoader::new(
        MonthMethaneEmissionsByFacilityLoader::new(pool.clone()),
        tokio::spawn,
    );
    let month_methane_emissions_by_site_id_loader = DataLoader::new(
        MonthMethaneEmissionsBySiteLoader::new(pool.clone()),
        tokio::spawn,
    );
    let month_methane_emissions_by_source_table_id_loader = DataLoader::new(
        MonthMethaneEmissionsBySourceTableLoader::new(pool.clone()),
        tokio::spawn,
    );
    let month_methane_emissions_by_creator_id_loader = DataLoader::new(
        CreatedMonthMethaneEmissionsLoader::new(pool.clone()),
        tokio::spawn,
    );
    let month_methane_emissions_by_updater_id_loader = DataLoader::new(
        UpdatedMonthMethaneEmissionsLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Compressor
    let compressor_by_id_loader =
        DataLoader::new(CompressorLoader::new(pool.clone()), tokio::spawn);
    let compressors_by_site_id_loader =
        DataLoader::new(SiteCompressorsLoader::new(pool.clone()), tokio::spawn);
    let compressors_by_creator_id_loader =
        DataLoader::new(CreatedCompressorsLoader::new(pool.clone()), tokio::spawn);
    let compressors_by_updater_id_loader =
        DataLoader::new(UpdatedCompressorsLoader::new(pool.clone()), tokio::spawn);

    // Compressor Seal
    let compressor_seal_by_id_loader =
        DataLoader::new(CompressorSealLoader::new(pool.clone()), tokio::spawn);
    let compressor_seals_by_creator_id_loader = DataLoader::new(
        CreatedCompressorSealsLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_seals_by_updater_id_loader = DataLoader::new(
        UpdatedCompressorSealsLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Compressor Seal Test
    let compressor_seal_test_by_id_loader =
        DataLoader::new(CompressorSealTestLoader::new(pool.clone()), tokio::spawn);
    let compressor_seal_tests_by_compressor_seal_id_loader = DataLoader::new(
        CompressorSealTestsByCompressorSealLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_seal_tests_by_survey_equipment_id_loader = DataLoader::new(
        CompressorSealTestsBySurveyEquipmentLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_seal_tests_by_creator_id_loader = DataLoader::new(
        CreatedCompressorSealTestsLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_seal_tests_by_updater_id_loader = DataLoader::new(
        UpdatedCompressorSealTestsLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Compressor Month Hours
    let compressor_controlled_characterization_by_id_loader = DataLoader::new(
        CompressorControlledCharacterizationLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_controlled_characterizations_by_compressor_id_loader = DataLoader::new(
        CompressorControlledCharacterizationsByCompressorLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_controlled_characterizations_by_creator_id_loader = DataLoader::new(
        CreatedCompressorControlledCharacterizationsLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_controlled_characterizations_by_updater_id_loader = DataLoader::new(
        UpdatedCompressorControlledCharacterizationsLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Compressor Month Hours
    let compressor_month_hours_by_id_loader =
        DataLoader::new(CompressorMonthHoursLoader::new(pool.clone()), tokio::spawn);
    let compressor_month_hours_by_compressor_id_loader = DataLoader::new(
        CompressorMonthHoursByCompressorLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_month_hours_by_creator_id_loader = DataLoader::new(
        CreatedCompressorMonthHoursLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_month_hours_by_updater_id_loader = DataLoader::new(
        UpdatedCompressorMonthHoursLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Compressor Seal Month Methane Emission Override
    let compressor_seal_month_methane_emission_override_by_id_loader = DataLoader::new(
        CompressorSealMonthMethaneEmissionOverrideLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_seal_month_methane_emission_overrides_by_compressor_seal_id_loader =
        DataLoader::new(
            CompressorSealMonthMethaneEmissionOverridesByCompressorSealLoader::new(pool.clone()),
            tokio::spawn,
        );
    let compressor_seal_month_methane_emission_overrides_by_creator_id_loader = DataLoader::new(
        CreatedCompressorSealMonthMethaneEmissionOverridesLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_seal_month_methane_emission_overrides_by_updater_id_loader = DataLoader::new(
        UpdatedCompressorSealMonthMethaneEmissionOverridesLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Compressor Blowdown
    let compressor_blowdown_by_id_loader =
        DataLoader::new(CompressorBlowdownLoader::new(pool.clone()), tokio::spawn);
    let compressor_blowdowns_by_compressor_id_loader = DataLoader::new(
        CompressorBlowdownsByCompressorLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_blowdowns_by_creator_id_loader = DataLoader::new(
        CreatedCompressorBlowdownsLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_blowdowns_by_updater_id_loader = DataLoader::new(
        UpdatedCompressorBlowdownsLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Compressor Blowdown Override
    let compressor_blowdown_override_by_id_loader = DataLoader::new(
        CompressorBlowdownOverrideLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_blowdown_overrides_by_compressor_id_loader = DataLoader::new(
        CompressorBlowdownOverridesByCompressorLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_blowdown_overrides_by_creator_id_loader = DataLoader::new(
        CreatedCompressorBlowdownOverridesLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_blowdown_overrides_by_updater_id_loader = DataLoader::new(
        UpdatedCompressorBlowdownOverridesLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Storage Tank
    let storage_tank_by_id_loader =
        DataLoader::new(StorageTankLoader::new(pool.clone()), tokio::spawn);
    let storage_tank_by_site_id_loader =
        DataLoader::new(SiteStorageTanksLoader::new(pool.clone()), tokio::spawn);
    let storage_tanks_by_creator_id_loader =
        DataLoader::new(CreatedStorageTanksLoader::new(pool.clone()), tokio::spawn);
    let storage_tanks_by_updater_id_loader =
        DataLoader::new(UpdatedStorageTanksLoader::new(pool.clone()), tokio::spawn);

    // Storage Tank Change
    let storage_tank_change_by_id_loader =
        DataLoader::new(StorageTankChangeLoader::new(pool.clone()), tokio::spawn);
    let storage_tank_changes_by_storage_tank_id_loader = DataLoader::new(
        StorageTankChangesByStorageTankLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_changes_by_creator_id_loader = DataLoader::new(
        CreatedStorageTankChangesLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_changes_by_updater_id_loader = DataLoader::new(
        UpdatedStorageTankChangesLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Storage Tank Controlled Characterization
    let storage_tank_controlled_characterization_by_id_loader = DataLoader::new(
        StorageTankControlledCharacterizationLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_controlled_characterizations_by_storage_tank_id_loader = DataLoader::new(
        StorageTankControlledCharacterizationsByStorageTankLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_controlled_characterizations_by_creator_id_loader = DataLoader::new(
        CreatedStorageTankControlledCharacterizationsLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_controlled_characterizations_by_updater_id_loader = DataLoader::new(
        UpdatedStorageTankControlledCharacterizationsLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Storage Tank Control Device Inactivity
    let storage_tank_control_device_inactivity_by_id_loader = DataLoader::new(
        StorageTankControlDeviceInactivityLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_control_device_inactivities_by_storage_tank_controlled_characterization_id_loader =
        DataLoader::new(
            StorageTankControlDeviceInactivitiesByStorageTankControlledCharacterizationLoader::new(
                pool.clone(),
            ),
            tokio::spawn,
        );
    let storage_tank_control_device_inactivities_by_creator_id_loader = DataLoader::new(
        CreatedStorageTankControlDeviceInactivitiesLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_control_device_inactivities_by_updater_id_loader = DataLoader::new(
        UpdatedStorageTankControlDeviceInactivitiesLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Storage Tank Month Oil Flow
    let storage_tank_month_liquid_hydrocarbon_entering_by_id_loader = DataLoader::new(
        StorageTankMonthLiquidHydrocarbonEnteringLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_month_liquid_hydrocarbon_entering_by_storage_tank_id_loader = DataLoader::new(
        StorageTankMonthLiquidHydrocarbonEnteringByStorageTankLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_month_liquid_hydrocarbon_entering_by_creator_id_loader = DataLoader::new(
        CreatedStorageTankMonthLiquidHydrocarbonEnteringLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_month_liquid_hydrocarbon_entering_by_updater_id_loader = DataLoader::new(
        UpdatedStorageTankMonthLiquidHydrocarbonEnteringLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Storage Tank Gas In Solution Factor Calculated
    let storage_tank_gas_in_solution_factor_calculated_by_id_loader = DataLoader::new(
        StorageTankGasInSolutionFactorCalculatedLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_gas_in_solution_factors_calculated_by_storage_tank_id_loader = DataLoader::new(
        StorageTankGasInSolutionFactorsCalculatedByStorageTankLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_gas_in_solution_factors_calculated_by_creator_id_loader = DataLoader::new(
        CreatedStorageTankGasInSolutionFactorsCalculatedLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_gas_in_solution_factors_calculated_by_updater_id_loader = DataLoader::new(
        UpdatedStorageTankGasInSolutionFactorsCalculatedLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Storage Tank Emission Survey
    let storage_tank_emission_survey_by_id_loader = DataLoader::new(
        StorageTankEmissionSurveyLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_emission_surveys_by_storage_tank_id_loader = DataLoader::new(
        StorageTankEmissionSurveysByStorageTankLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_emission_surveys_by_survey_equipment_id_loader = DataLoader::new(
        StorageTankEmissionSurveysBySurveyEquipmentLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_emission_surveys_by_creator_id_loader = DataLoader::new(
        CreatedStorageTankEmissionSurveysLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_emission_surveys_by_updater_id_loader = DataLoader::new(
        UpdatedStorageTankEmissionSurveysLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Storage Tank Month Methane Emission Override
    let storage_tank_month_methane_emission_override_by_id_loader = DataLoader::new(
        StorageTankMonthMethaneEmissionOverrideLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_month_methane_emission_overrides_by_storage_tank_id_loader = DataLoader::new(
        StorageTankMonthMethaneEmissionOverridesByStorageTankLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_month_methane_emission_overrides_by_creator_id_loader = DataLoader::new(
        CreatedStorageTankMonthMethaneEmissionOverridesLoader::new(pool.clone()),
        tokio::spawn,
    );
    let storage_tank_month_methane_emission_overrides_by_updater_id_loader = DataLoader::new(
        UpdatedStorageTankMonthMethaneEmissionOverridesLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Gas Analysis
    let gas_analysis_by_id_loader =
        DataLoader::new(GasAnalysisLoader::new(pool.clone()), tokio::spawn);
    let gas_analyses_by_facility_id_loader =
        DataLoader::new(GasAnalysesByFacilityLoader::new(pool.clone()), tokio::spawn);
    let gas_analyses_by_creator_id_loader =
        DataLoader::new(CreatedGasAnalysesLoader::new(pool.clone()), tokio::spawn);
    let gas_analyses_by_updater_id_loader =
        DataLoader::new(UpdatedGasAnalysesLoader::new(pool.clone()), tokio::spawn);

    // Gas Analysis Calculated Param
    let gas_analysis_calculated_param_by_id_loader = DataLoader::new(
        GasAnalysisCalculatedParamLoader::new(pool.clone()),
        tokio::spawn,
    );
    let gas_analysis_calculated_params_by_creator_id_loader = DataLoader::new(
        CreatedGasAnalysisCalculatedParamsLoader::new(pool.clone()),
        tokio::spawn,
    );
    let gas_analysis_calculated_params_by_updater_id_loader = DataLoader::new(
        UpdatedGasAnalysisCalculatedParamsLoader::new(pool.clone()),
        tokio::spawn,
    );

    loaders.insert(user_by_id_loader);

    loaders.insert(facility_by_id_loader);
    loaders.insert(facilities_by_creator_id_loader);
    loaders.insert(facilities_by_updater_id_loader);

    loaders.insert(site_by_id_loader);
    loaders.insert(sites_by_facility_id_loader);
    loaders.insert(sites_by_creator_id_loader);
    loaders.insert(sites_by_updater_id_loader);

    loaders.insert(survey_equipment_by_id_loader);
    loaders.insert(survey_equipment_by_creator_id_loader);
    loaders.insert(survey_equipment_by_updater_id_loader);

    loaders.insert(non_level_controller_by_id_loader);
    loaders.insert(non_level_controllers_by_creator_id_loader);
    loaders.insert(non_level_controllers_by_updater_id_loader);
    loaders.insert(non_level_controllers_by_site_id_loader);
    loaders.insert(non_level_controllers_by_manufacturer_id_loader);

    loaders.insert(device_manufacturer_by_id_loader);
    loaders.insert(device_manufacturers_by_creator_id_loader);
    loaders.insert(device_manufacturers_by_updater_id_loader);

    loaders.insert(non_level_controller_change_by_id_loader);
    loaders.insert(non_level_controller_changes_by_non_level_controller_id_loader);
    loaders.insert(non_level_controller_changes_by_creator_id_loader);
    loaders.insert(non_level_controller_changes_by_updater_id_loader);

    loaders.insert(non_level_controller_month_hours_by_id_loader);
    loaders.insert(non_level_controller_month_hours_by_non_level_controller_id_loader);
    loaders.insert(non_level_controller_month_hours_by_creator_id_loader);
    loaders.insert(non_level_controller_month_hours_by_updater_id_loader);

    loaders.insert(non_level_controller_month_methane_emission_override_by_id_loader);
    loaders.insert(
        non_level_controller_month_methane_emission_overrides_by_non_level_controller_id_loader,
    );
    loaders.insert(non_level_controller_month_methane_emission_overrides_by_creator_id_loader);
    loaders.insert(non_level_controller_month_methane_emission_overrides_by_updater_id_loader);

    loaders.insert(level_controller_by_id_loader);
    loaders.insert(level_controllers_by_creator_id_loader);
    loaders.insert(level_controllers_by_updater_id_loader);
    loaders.insert(level_controllers_by_site_id_loader);
    loaders.insert(level_controllers_by_manufacturer_id_loader);

    loaders.insert(level_controller_actuation_frequency_by_id_loader);
    loaders.insert(level_controller_actuation_frequencies_by_level_controller_id_loader);
    loaders.insert(level_controller_actuation_frequencies_by_creator_id_loader);
    loaders.insert(level_controller_actuation_frequencies_by_updater_id_loader);

    loaders.insert(level_controller_change_by_id_loader);
    loaders.insert(level_controller_changes_by_level_controller_id_loader);
    loaders.insert(level_controller_changes_by_creator_id_loader);
    loaders.insert(level_controller_changes_by_updater_id_loader);

    loaders.insert(level_controller_month_hours_by_id_loader);
    loaders.insert(level_controller_month_hours_by_level_controller_id_loader);
    loaders.insert(level_controller_month_hours_by_creator_id_loader);
    loaders.insert(level_controller_month_hours_by_updater_id_loader);

    loaders.insert(level_controller_month_methane_emission_override_by_id_loader);
    loaders.insert(level_controller_month_methane_emission_overrides_by_level_controller_id_loader);
    loaders.insert(level_controller_month_methane_emission_overrides_by_creator_id_loader);
    loaders.insert(level_controller_month_methane_emission_overrides_by_updater_id_loader);

    loaders.insert(month_methane_emission_by_id_loader);
    loaders.insert(month_methane_emissions_by_facility_id_loader);
    loaders.insert(month_methane_emissions_by_site_id_loader);
    loaders.insert(month_methane_emissions_by_source_table_id_loader);
    loaders.insert(month_methane_emissions_by_creator_id_loader);
    loaders.insert(month_methane_emissions_by_updater_id_loader);

    loaders.insert(compressor_by_id_loader);
    loaders.insert(compressors_by_creator_id_loader);
    loaders.insert(compressors_by_updater_id_loader);
    loaders.insert(compressors_by_site_id_loader);

    loaders.insert(compressor_seal_by_id_loader);
    loaders.insert(compressor_seals_by_creator_id_loader);
    loaders.insert(compressor_seals_by_updater_id_loader);

    loaders.insert(compressor_seal_test_by_id_loader);
    loaders.insert(compressor_seal_tests_by_compressor_seal_id_loader);
    loaders.insert(compressor_seal_tests_by_survey_equipment_id_loader);
    loaders.insert(compressor_seal_tests_by_creator_id_loader);
    loaders.insert(compressor_seal_tests_by_updater_id_loader);

    loaders.insert(compressor_controlled_characterization_by_id_loader);
    loaders.insert(compressor_controlled_characterizations_by_compressor_id_loader);
    loaders.insert(compressor_controlled_characterizations_by_creator_id_loader);
    loaders.insert(compressor_controlled_characterizations_by_updater_id_loader);

    loaders.insert(compressor_month_hours_by_id_loader);
    loaders.insert(compressor_month_hours_by_compressor_id_loader);
    loaders.insert(compressor_month_hours_by_creator_id_loader);
    loaders.insert(compressor_month_hours_by_updater_id_loader);

    loaders.insert(compressor_seal_month_methane_emission_override_by_id_loader);
    loaders.insert(compressor_seal_month_methane_emission_overrides_by_compressor_seal_id_loader);
    loaders.insert(compressor_seal_month_methane_emission_overrides_by_creator_id_loader);
    loaders.insert(compressor_seal_month_methane_emission_overrides_by_updater_id_loader);

    loaders.insert(compressor_blowdown_by_id_loader);
    loaders.insert(compressor_blowdowns_by_compressor_id_loader);
    loaders.insert(compressor_blowdowns_by_creator_id_loader);
    loaders.insert(compressor_blowdowns_by_updater_id_loader);

    loaders.insert(compressor_blowdown_override_by_id_loader);
    loaders.insert(compressor_blowdown_overrides_by_compressor_id_loader);
    loaders.insert(compressor_blowdown_overrides_by_creator_id_loader);
    loaders.insert(compressor_blowdown_overrides_by_updater_id_loader);

    loaders.insert(storage_tank_by_id_loader);
    loaders.insert(storage_tanks_by_creator_id_loader);
    loaders.insert(storage_tanks_by_updater_id_loader);
    loaders.insert(storage_tank_by_site_id_loader);

    loaders.insert(storage_tank_change_by_id_loader);
    loaders.insert(storage_tank_changes_by_storage_tank_id_loader);
    loaders.insert(storage_tank_changes_by_creator_id_loader);
    loaders.insert(storage_tank_changes_by_updater_id_loader);

    loaders.insert(storage_tank_controlled_characterization_by_id_loader);
    loaders.insert(storage_tank_controlled_characterizations_by_storage_tank_id_loader);
    loaders.insert(storage_tank_controlled_characterizations_by_creator_id_loader);
    loaders.insert(storage_tank_controlled_characterizations_by_updater_id_loader);

    loaders.insert(storage_tank_control_device_inactivity_by_id_loader);
    loaders.insert(storage_tank_control_device_inactivities_by_storage_tank_controlled_characterization_id_loader);
    loaders.insert(storage_tank_control_device_inactivities_by_creator_id_loader);
    loaders.insert(storage_tank_control_device_inactivities_by_updater_id_loader);

    loaders.insert(storage_tank_month_liquid_hydrocarbon_entering_by_id_loader);
    loaders.insert(storage_tank_month_liquid_hydrocarbon_entering_by_storage_tank_id_loader);
    loaders.insert(storage_tank_month_liquid_hydrocarbon_entering_by_creator_id_loader);
    loaders.insert(storage_tank_month_liquid_hydrocarbon_entering_by_updater_id_loader);

    loaders.insert(storage_tank_gas_in_solution_factor_calculated_by_id_loader);
    loaders.insert(storage_tank_gas_in_solution_factors_calculated_by_storage_tank_id_loader);
    loaders.insert(storage_tank_gas_in_solution_factors_calculated_by_creator_id_loader);
    loaders.insert(storage_tank_gas_in_solution_factors_calculated_by_updater_id_loader);

    loaders.insert(storage_tank_emission_survey_by_id_loader);
    loaders.insert(storage_tank_emission_surveys_by_storage_tank_id_loader);
    loaders.insert(storage_tank_emission_surveys_by_survey_equipment_id_loader);
    loaders.insert(storage_tank_emission_surveys_by_creator_id_loader);
    loaders.insert(storage_tank_emission_surveys_by_updater_id_loader);

    loaders.insert(storage_tank_month_methane_emission_override_by_id_loader);
    loaders.insert(storage_tank_month_methane_emission_overrides_by_storage_tank_id_loader);
    loaders.insert(storage_tank_month_methane_emission_overrides_by_creator_id_loader);
    loaders.insert(storage_tank_month_methane_emission_overrides_by_updater_id_loader);

    loaders.insert(gas_analysis_by_id_loader);
    loaders.insert(gas_analyses_by_facility_id_loader);
    loaders.insert(gas_analyses_by_creator_id_loader);
    loaders.insert(gas_analyses_by_updater_id_loader);

    loaders.insert(gas_analysis_calculated_param_by_id_loader);
    loaders.insert(gas_analysis_calculated_params_by_creator_id_loader);
    loaders.insert(gas_analysis_calculated_params_by_updater_id_loader);

    loaders
}
