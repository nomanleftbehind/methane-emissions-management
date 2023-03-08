use super::{
    compressor_blowdown_loader::{
        CompressorBlowdownLoader, CompressorBlowdownsByCompressorLoader,
        CreatedCompressorBlowdownsLoader, UpdatedCompressorBlowdownsLoader,
    },
    compressor_change_loader::{
        CompressorChangeLoader, CompressorChangesByCompressorLoader,
        CreatedCompressorChangesLoader, UpdatedCompressorChangesLoader,
    },
    compressor_loader::{
        CompressorLoader, CreatedCompressorsLoader, FacilityCompressorsLoader,
        UpdatedCompressorsLoader,
    },
    compressor_month_hours_loader::{
        CompressorMonthHoursByCompressorLoader, CompressorMonthHoursLoader,
        CreatedCompressorMonthHoursLoader, UpdatedCompressorMonthHoursLoader,
    },
    compressor_month_vent_loader::{
        CompressorMonthVentLoader, CompressorMonthVentsByCompressorLoader,
        CreatedCompressorMonthVentsLoader, UpdatedCompressorMonthVentsLoader,
    },
    compressor_month_vent_override_loader::{
        CompressorMonthVentOverrideLoader, CompressorMonthVentOverridesByCompressorLoader,
        CreatedCompressorMonthVentOverridesLoader, UpdatedCompressorMonthVentOverridesLoader,
    },
    controller_application_loader::{
        ControllerApplicationLoader, CreatedControllerApplicationsLoader,
        UpdatedControllerApplicationsLoader,
    },
    controller_change_loader::{
        ControllerChangeLoader, ControllerChangesByControllerLoader,
        CreatedControllerChangesLoader, UpdatedControllerChangesLoader,
    },
    controller_loader::{
        ControllerLoader, ControllersByApplicationLoader, ControllersByManufacturerLoader,
        CreatedControllersLoader, FacilityControllersLoader, UpdatedControllersLoader,
    },
    controller_manufacturer_loader::{
        ControllerManufacturerLoader, CreatedControllerManufacturersLoader,
        UpdatedControllerManufacturersLoader,
    },
    controller_month_hours_loader::{
        ControllerMonthHoursByControllerLoader, ControllerMonthHoursLoader,
        CreatedControllerMonthHoursLoader, UpdatedControllerMonthHoursLoader,
    },
    controller_month_vent_loader::{
        ControllerMonthVentLoader, ControllerMonthVentsByControllerLoader,
        CreatedControllerMonthVentsLoader, UpdatedControllerMonthVentsLoader,
    },
    controller_month_vent_override_loader::{
        ControllerMonthVentOverrideLoader, ControllerMonthVentOverridesByControllerLoader,
        CreatedControllerMonthVentOverridesLoader, UpdatedControllerMonthVentOverridesLoader,
    },
    facility_loader::{CreatedFacilitiesLoader, FacilityLoader, UpdatedFacilitiesLoader},
    gas_analysis_calculated_param_loader::{
        CreatedGasAnalysisCalculatedParamsLoader, GasAnalysisCalculatedParamLoader,
        UpdatedGasAnalysisCalculatedParamsLoader,
    },
    gas_analysis_loader::{
        CreatedGasAnalysesLoader, GasAnalysesByFacilityLoader, GasAnalysisLoader,
        UpdatedGasAnalysesLoader,
    },
    tank_farm_change_loader::{
        CreatedTankFarmChangesLoader, TankFarmChangeLoader, TankFarmChangesByTankFarmLoader,
        UpdatedTankFarmChangesLoader,
    },
    tank_farm_loader::{
        CreatedTankFarmsLoader, FacilityTankFarmLoader, TankFarmLoader, UpdatedTankFarmsLoader,
    },
    tank_farm_month_oil_flow_loader::{
        CreatedTankFarmMonthOilFlowsLoader, TankFarmMonthOilFlowLoader,
        TankFarmMonthOilFlowsByTankFarmLoader, UpdatedTankFarmMonthOilFlowsLoader,
    },
    tank_farm_month_vent_loader::{
        CreatedTankFarmMonthVentsLoader, TankFarmMonthVentLoader,
        TankFarmMonthVentsByTankFarmLoader, UpdatedTankFarmMonthVentsLoader,
    },
    tank_farm_month_vent_override_loader::{
        CreatedTankFarmMonthVentOverridesLoader, TankFarmMonthVentOverrideLoader,
        TankFarmMonthVentOverridesByTankFarmLoader, UpdatedTankFarmMonthVentOverridesLoader,
    },
    tank_farm_vent_factor_loader::{
        CreatedTankFarmVentFactorsCalculatedLoader, TankFarmVentFactorCalculatedLoader,
        TankFarmVentFactorsCalculatedByTankFarmLoader, UpdatedTankFarmVentFactorsCalculatedLoader,
    },
    user_loader::UserLoader,
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

    // Controllers
    let controller_by_id_loader =
        DataLoader::new(ControllerLoader::new(pool.clone()), tokio::spawn);
    let controllers_by_creator_id_loader =
        DataLoader::new(CreatedControllersLoader::new(pool.clone()), tokio::spawn);
    let controllers_by_updater_id_loader =
        DataLoader::new(UpdatedControllersLoader::new(pool.clone()), tokio::spawn);
    let controllers_by_facility_id_loader =
        DataLoader::new(FacilityControllersLoader::new(pool.clone()), tokio::spawn);
    let controllers_by_application_id_loader = DataLoader::new(
        ControllersByApplicationLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controllers_by_manufacturer_id_loader = DataLoader::new(
        ControllersByManufacturerLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Controller Applications
    let controller_applications_by_creator_id_loader = DataLoader::new(
        CreatedControllerApplicationsLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_applications_by_updater_id_loader = DataLoader::new(
        UpdatedControllerApplicationsLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_application_by_id_loader =
        DataLoader::new(ControllerApplicationLoader::new(pool.clone()), tokio::spawn);

    // Controller Manufacturers
    let controller_manufacturer_by_id_loader = DataLoader::new(
        ControllerManufacturerLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_manufacturers_by_creator_id_loader = DataLoader::new(
        CreatedControllerManufacturersLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_manufacturers_by_updater_id_loader = DataLoader::new(
        UpdatedControllerManufacturersLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Controller Change
    let controller_change_by_id_loader =
        DataLoader::new(ControllerChangeLoader::new(pool.clone()), tokio::spawn);
    let controller_changes_by_controller_id_loader = DataLoader::new(
        ControllerChangesByControllerLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_changes_by_creator_id_loader = DataLoader::new(
        CreatedControllerChangesLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_changes_by_updater_id_loader = DataLoader::new(
        UpdatedControllerChangesLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Controller Month Hours
    let controller_month_hours_by_id_loader =
        DataLoader::new(ControllerMonthHoursLoader::new(pool.clone()), tokio::spawn);
    let controller_month_hours_by_controller_id_loader = DataLoader::new(
        ControllerMonthHoursByControllerLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_month_hours_by_creator_id_loader = DataLoader::new(
        CreatedControllerMonthHoursLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_month_hours_by_updater_id_loader = DataLoader::new(
        UpdatedControllerMonthHoursLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Controller Month Vent Override
    let controller_month_vent_override_by_id_loader = DataLoader::new(
        ControllerMonthVentOverrideLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_month_vent_overrides_by_controller_id_loader = DataLoader::new(
        ControllerMonthVentOverridesByControllerLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_month_vent_overrides_by_creator_id_loader = DataLoader::new(
        CreatedControllerMonthVentOverridesLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_month_vent_overrides_by_updater_id_loader = DataLoader::new(
        UpdatedControllerMonthVentOverridesLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Controller Month Vent
    let controller_month_vent_by_id_loader =
        DataLoader::new(ControllerMonthVentLoader::new(pool.clone()), tokio::spawn);
    let controller_month_vents_by_controller_id_loader = DataLoader::new(
        ControllerMonthVentsByControllerLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_month_vents_by_creator_id_loader = DataLoader::new(
        CreatedControllerMonthVentsLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_month_vents_by_updater_id_loader = DataLoader::new(
        UpdatedControllerMonthVentsLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Compressor
    let compressor_by_id_loader =
        DataLoader::new(CompressorLoader::new(pool.clone()), tokio::spawn);
    let compressors_by_creator_id_loader =
        DataLoader::new(CreatedCompressorsLoader::new(pool.clone()), tokio::spawn);
    let compressors_by_updater_id_loader =
        DataLoader::new(UpdatedCompressorsLoader::new(pool.clone()), tokio::spawn);
    let compressors_by_facility_id_loader =
        DataLoader::new(FacilityCompressorsLoader::new(pool.clone()), tokio::spawn);

    // Compressor Change
    let compressor_change_by_id_loader =
        DataLoader::new(CompressorChangeLoader::new(pool.clone()), tokio::spawn);
    let compressor_changes_by_compressor_id_loader = DataLoader::new(
        CompressorChangesByCompressorLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_changes_by_creator_id_loader = DataLoader::new(
        CreatedCompressorChangesLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_changes_by_updater_id_loader = DataLoader::new(
        UpdatedCompressorChangesLoader::new(pool.clone()),
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

    // Compressor Month Vent Override
    let compressor_month_vent_override_by_id_loader = DataLoader::new(
        CompressorMonthVentOverrideLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_month_vent_overrides_by_compressor_id_loader = DataLoader::new(
        CompressorMonthVentOverridesByCompressorLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_month_vent_overrides_by_creator_id_loader = DataLoader::new(
        CreatedCompressorMonthVentOverridesLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_month_vent_overrides_by_updater_id_loader = DataLoader::new(
        UpdatedCompressorMonthVentOverridesLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Compressor Month Vent
    let compressor_month_vent_by_id_loader =
        DataLoader::new(CompressorMonthVentLoader::new(pool.clone()), tokio::spawn);
    let compressor_month_vents_by_compressor_id_loader = DataLoader::new(
        CompressorMonthVentsByCompressorLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_month_vents_by_creator_id_loader = DataLoader::new(
        CreatedCompressorMonthVentsLoader::new(pool.clone()),
        tokio::spawn,
    );
    let compressor_month_vents_by_updater_id_loader = DataLoader::new(
        UpdatedCompressorMonthVentsLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Tank Farm
    let tank_farm_by_id_loader = DataLoader::new(TankFarmLoader::new(pool.clone()), tokio::spawn);
    let tank_farms_by_creator_id_loader =
        DataLoader::new(CreatedTankFarmsLoader::new(pool.clone()), tokio::spawn);
    let tank_farms_by_updater_id_loader =
        DataLoader::new(UpdatedTankFarmsLoader::new(pool.clone()), tokio::spawn);
    let tank_farm_by_facility_id_loader =
        DataLoader::new(FacilityTankFarmLoader::new(pool.clone()), tokio::spawn);

    // Tank Farm Change
    let tank_farm_change_by_id_loader =
        DataLoader::new(TankFarmChangeLoader::new(pool.clone()), tokio::spawn);
    let tank_farm_changes_by_tank_farm_id_loader = DataLoader::new(
        TankFarmChangesByTankFarmLoader::new(pool.clone()),
        tokio::spawn,
    );
    let tank_farm_changes_by_creator_id_loader = DataLoader::new(
        CreatedTankFarmChangesLoader::new(pool.clone()),
        tokio::spawn,
    );
    let tank_farm_changes_by_updater_id_loader = DataLoader::new(
        UpdatedTankFarmChangesLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Tank Farm Month Oil Flow
    let tank_farm_month_oil_flow_by_id_loader =
        DataLoader::new(TankFarmMonthOilFlowLoader::new(pool.clone()), tokio::spawn);
    let tank_farm_month_oil_flows_by_tank_farm_id_loader = DataLoader::new(
        TankFarmMonthOilFlowsByTankFarmLoader::new(pool.clone()),
        tokio::spawn,
    );
    let tank_farm_month_oil_flows_by_creator_id_loader = DataLoader::new(
        CreatedTankFarmMonthOilFlowsLoader::new(pool.clone()),
        tokio::spawn,
    );
    let tank_farm_month_oil_flows_by_updater_id_loader = DataLoader::new(
        UpdatedTankFarmMonthOilFlowsLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Tank Farm Vent Factor Calculated
    let tank_farm_vent_factor_calculated_by_id_loader = DataLoader::new(
        TankFarmVentFactorCalculatedLoader::new(pool.clone()),
        tokio::spawn,
    );
    let tank_farm_vent_factors_calculated_by_tank_farm_id_loader = DataLoader::new(
        TankFarmVentFactorsCalculatedByTankFarmLoader::new(pool.clone()),
        tokio::spawn,
    );
    let tank_farm_vent_factors_calculated_by_creator_id_loader = DataLoader::new(
        CreatedTankFarmVentFactorsCalculatedLoader::new(pool.clone()),
        tokio::spawn,
    );
    let tank_farm_vent_factors_calculated_by_updater_id_loader = DataLoader::new(
        UpdatedTankFarmVentFactorsCalculatedLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Tank Farm Month Vent Override
    let tank_farm_month_vent_override_by_id_loader = DataLoader::new(
        TankFarmMonthVentOverrideLoader::new(pool.clone()),
        tokio::spawn,
    );
    let tank_farm_month_vent_overrides_by_tank_farm_id_loader = DataLoader::new(
        TankFarmMonthVentOverridesByTankFarmLoader::new(pool.clone()),
        tokio::spawn,
    );
    let tank_farm_month_vent_overrides_by_creator_id_loader = DataLoader::new(
        CreatedTankFarmMonthVentOverridesLoader::new(pool.clone()),
        tokio::spawn,
    );
    let tank_farm_month_vent_overrides_by_updater_id_loader = DataLoader::new(
        UpdatedTankFarmMonthVentOverridesLoader::new(pool.clone()),
        tokio::spawn,
    );

    // Tank Farm Month Vent
    let tank_farm_month_vent_by_id_loader =
        DataLoader::new(TankFarmMonthVentLoader::new(pool.clone()), tokio::spawn);
    let tank_farm_month_vents_by_tank_farm_id_loader = DataLoader::new(
        TankFarmMonthVentsByTankFarmLoader::new(pool.clone()),
        tokio::spawn,
    );
    let tank_farm_month_vents_by_creator_id_loader = DataLoader::new(
        CreatedTankFarmMonthVentsLoader::new(pool.clone()),
        tokio::spawn,
    );
    let tank_farm_month_vents_by_updater_id_loader = DataLoader::new(
        UpdatedTankFarmMonthVentsLoader::new(pool.clone()),
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

    loaders.insert(controller_by_id_loader);
    loaders.insert(controllers_by_creator_id_loader);
    loaders.insert(controllers_by_updater_id_loader);
    loaders.insert(controllers_by_facility_id_loader);
    loaders.insert(controllers_by_application_id_loader);
    loaders.insert(controllers_by_manufacturer_id_loader);

    loaders.insert(controller_application_by_id_loader);
    loaders.insert(controller_applications_by_creator_id_loader);
    loaders.insert(controller_applications_by_updater_id_loader);

    loaders.insert(controller_manufacturer_by_id_loader);
    loaders.insert(controller_manufacturers_by_creator_id_loader);
    loaders.insert(controller_manufacturers_by_updater_id_loader);

    loaders.insert(controller_change_by_id_loader);
    loaders.insert(controller_changes_by_controller_id_loader);
    loaders.insert(controller_changes_by_creator_id_loader);
    loaders.insert(controller_changes_by_updater_id_loader);

    loaders.insert(controller_month_hours_by_id_loader);
    loaders.insert(controller_month_hours_by_controller_id_loader);
    loaders.insert(controller_month_hours_by_creator_id_loader);
    loaders.insert(controller_month_hours_by_updater_id_loader);

    loaders.insert(controller_month_vent_override_by_id_loader);
    loaders.insert(controller_month_vent_overrides_by_controller_id_loader);
    loaders.insert(controller_month_vent_overrides_by_creator_id_loader);
    loaders.insert(controller_month_vent_overrides_by_updater_id_loader);

    loaders.insert(controller_month_vent_by_id_loader);
    loaders.insert(controller_month_vents_by_controller_id_loader);
    loaders.insert(controller_month_vents_by_creator_id_loader);
    loaders.insert(controller_month_vents_by_updater_id_loader);

    loaders.insert(compressor_by_id_loader);
    loaders.insert(compressors_by_creator_id_loader);
    loaders.insert(compressors_by_updater_id_loader);
    loaders.insert(compressors_by_facility_id_loader);

    loaders.insert(compressor_change_by_id_loader);
    loaders.insert(compressor_changes_by_compressor_id_loader);
    loaders.insert(compressor_changes_by_creator_id_loader);
    loaders.insert(compressor_changes_by_updater_id_loader);

    loaders.insert(compressor_month_hours_by_id_loader);
    loaders.insert(compressor_month_hours_by_compressor_id_loader);
    loaders.insert(compressor_month_hours_by_creator_id_loader);
    loaders.insert(compressor_month_hours_by_updater_id_loader);

    loaders.insert(compressor_blowdown_by_id_loader);
    loaders.insert(compressor_blowdowns_by_compressor_id_loader);
    loaders.insert(compressor_blowdowns_by_creator_id_loader);
    loaders.insert(compressor_blowdowns_by_updater_id_loader);

    loaders.insert(compressor_month_vent_override_by_id_loader);
    loaders.insert(compressor_month_vent_overrides_by_compressor_id_loader);
    loaders.insert(compressor_month_vent_overrides_by_creator_id_loader);
    loaders.insert(compressor_month_vent_overrides_by_updater_id_loader);

    loaders.insert(compressor_month_vent_by_id_loader);
    loaders.insert(compressor_month_vents_by_compressor_id_loader);
    loaders.insert(compressor_month_vents_by_creator_id_loader);
    loaders.insert(compressor_month_vents_by_updater_id_loader);

    loaders.insert(tank_farm_by_id_loader);
    loaders.insert(tank_farms_by_creator_id_loader);
    loaders.insert(tank_farms_by_updater_id_loader);
    loaders.insert(tank_farm_by_facility_id_loader);

    loaders.insert(tank_farm_change_by_id_loader);
    loaders.insert(tank_farm_changes_by_tank_farm_id_loader);
    loaders.insert(tank_farm_changes_by_creator_id_loader);
    loaders.insert(tank_farm_changes_by_updater_id_loader);

    loaders.insert(tank_farm_month_oil_flow_by_id_loader);
    loaders.insert(tank_farm_month_oil_flows_by_tank_farm_id_loader);
    loaders.insert(tank_farm_month_oil_flows_by_creator_id_loader);
    loaders.insert(tank_farm_month_oil_flows_by_updater_id_loader);

    loaders.insert(tank_farm_vent_factor_calculated_by_id_loader);
    loaders.insert(tank_farm_vent_factors_calculated_by_tank_farm_id_loader);
    loaders.insert(tank_farm_vent_factors_calculated_by_creator_id_loader);
    loaders.insert(tank_farm_vent_factors_calculated_by_updater_id_loader);

    loaders.insert(tank_farm_month_vent_override_by_id_loader);
    loaders.insert(tank_farm_month_vent_overrides_by_tank_farm_id_loader);
    loaders.insert(tank_farm_month_vent_overrides_by_creator_id_loader);
    loaders.insert(tank_farm_month_vent_overrides_by_updater_id_loader);

    loaders.insert(tank_farm_month_vent_by_id_loader);
    loaders.insert(tank_farm_month_vents_by_tank_farm_id_loader);
    loaders.insert(tank_farm_month_vents_by_creator_id_loader);
    loaders.insert(tank_farm_month_vents_by_updater_id_loader);

    loaders.insert(gas_analysis_by_id_loader);
    loaders.insert(gas_analyses_by_facility_id_loader);
    loaders.insert(gas_analyses_by_creator_id_loader);
    loaders.insert(gas_analyses_by_updater_id_loader);

    loaders.insert(gas_analysis_calculated_param_by_id_loader);
    loaders.insert(gas_analysis_calculated_params_by_creator_id_loader);
    loaders.insert(gas_analysis_calculated_params_by_updater_id_loader);

    loaders
}