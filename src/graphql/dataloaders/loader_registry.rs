use super::{
    compressor_change_loader::{
        CompressorChangeLoader, CompressorChangesByCompressorLoader,
        CreatedCompressorChangesLoader, UpdatedCompressorChangesLoader,
    },
    compressor_loader::{
        CompressorLoader, CreatedCompressorsLoader, FacilityCompressorsLoader,
        UpdatedCompressorsLoader,
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
    facility_loader::{CreatedFacilitiesLoader, FacilityLoader, UpdatedFacilitiesLoader},
    gas_analysis_loader::{
        CreatedGasAnalysesLoader, GasAnalysesByFacilityLoader, GasAnalysisLoader,
        UpdatedGasAnalysesLoader,
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

    // Facility
    let facility_by_id_loader = DataLoader::new(FacilityLoader::new(pool.clone()), tokio::spawn);
    let facilities_by_creator_id_loader =
        DataLoader::new(CreatedFacilitiesLoader::new(pool.clone()), tokio::spawn);
    let facilities_by_updater_id_loader =
        DataLoader::new(UpdatedFacilitiesLoader::new(pool.clone()), tokio::spawn);

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

    // Gas Analysis
    let gas_analysis_by_id_loader =
        DataLoader::new(GasAnalysisLoader::new(pool.clone()), tokio::spawn);
    let gas_analyses_by_facility_id_loader =
        DataLoader::new(GasAnalysesByFacilityLoader::new(pool.clone()), tokio::spawn);
    let gas_analyses_by_creator_id_loader =
        DataLoader::new(CreatedGasAnalysesLoader::new(pool.clone()), tokio::spawn);
    let gas_analyses_by_updater_id_loader =
        DataLoader::new(UpdatedGasAnalysesLoader::new(pool.clone()), tokio::spawn);

    loaders.insert(user_by_id_loader);

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

    loaders.insert(controller_month_vent_by_id_loader);
    loaders.insert(controller_month_vents_by_controller_id_loader);
    loaders.insert(controller_month_vents_by_creator_id_loader);
    loaders.insert(controller_month_vents_by_updater_id_loader);

    loaders.insert(facility_by_id_loader);
    loaders.insert(facilities_by_creator_id_loader);
    loaders.insert(facilities_by_updater_id_loader);

    loaders.insert(compressor_by_id_loader);
    loaders.insert(compressors_by_creator_id_loader);
    loaders.insert(compressors_by_updater_id_loader);
    loaders.insert(compressors_by_facility_id_loader);

    loaders.insert(compressor_change_by_id_loader);
    loaders.insert(compressor_changes_by_compressor_id_loader);
    loaders.insert(compressor_changes_by_creator_id_loader);
    loaders.insert(compressor_changes_by_updater_id_loader);

    loaders.insert(gas_analysis_by_id_loader);
    loaders.insert(gas_analyses_by_facility_id_loader);
    loaders.insert(gas_analyses_by_creator_id_loader);
    loaders.insert(gas_analyses_by_updater_id_loader);

    loaders
}
