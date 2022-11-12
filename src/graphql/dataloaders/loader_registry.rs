use super::{
    controller_change_loader::{
        ControllerChangeLoader, ControllerChangesByControllerLoader,
        CreatedControllerChangesLoader, UpdatedControllerChangesLoader,
    },
    controller_function_loader::{
        ControllerFunctionLoader, CreatedControllerFunctionsLoader,
        UpdatedControllerFunctionsLoader,
    },
    controller_loader::{
        ControllersByFunctionLoader, ControllerLoader,
        ControllersByManufacturerLoader, CreatedControllersLoader,
        FacilityControllersLoader, UpdatedControllersLoader,
    },
    controller_manufacturer_loader::{
        ControllerManufacturerLoader, CreatedControllerManufacturersLoader,
        UpdatedControllerManufacturersLoader,
    },
    facility_loader::{CreatedFacilitiesLoader, FacilityLoader, UpdatedFacilitiesLoader},
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

    let user_by_id_loader = DataLoader::new(UserLoader::new(pool.clone()), tokio::spawn);

    let controller_by_id_loader =
        DataLoader::new(ControllerLoader::new(pool.clone()), tokio::spawn);
    let controllers_by_creator_id_loader =
        DataLoader::new(CreatedControllersLoader::new(pool.clone()), tokio::spawn);
    let controllers_by_updater_id_loader =
        DataLoader::new(UpdatedControllersLoader::new(pool.clone()), tokio::spawn);
    let controllers_by_facility_id_loader =
        DataLoader::new(FacilityControllersLoader::new(pool.clone()), tokio::spawn);
    let controllers_by_function_id_loader = DataLoader::new(
        ControllersByFunctionLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controllers_by_manufacturer_id_loader = DataLoader::new(
        ControllersByManufacturerLoader::new(pool.clone()),
        tokio::spawn,
    );

    let controller_functions_by_creator_id_loader = DataLoader::new(
        CreatedControllerFunctionsLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_functions_by_updater_id_loader = DataLoader::new(
        UpdatedControllerFunctionsLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_function_by_id_loader =
        DataLoader::new(ControllerFunctionLoader::new(pool.clone()), tokio::spawn);

    let controller_manufacturers_by_creator_id_loader = DataLoader::new(
        CreatedControllerManufacturersLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_manufacturers_by_updater_id_loader = DataLoader::new(
        UpdatedControllerManufacturersLoader::new(pool.clone()),
        tokio::spawn,
    );
    let controller_manufacturer_by_id_loader = DataLoader::new(
        ControllerManufacturerLoader::new(pool.clone()),
        tokio::spawn,
    );

    let facility_by_id_loader = DataLoader::new(FacilityLoader::new(pool.clone()), tokio::spawn);
    let facilities_by_creator_id_loader =
        DataLoader::new(CreatedFacilitiesLoader::new(pool.clone()), tokio::spawn);
    let facilities_by_updater_id_loader =
        DataLoader::new(UpdatedFacilitiesLoader::new(pool.clone()), tokio::spawn);

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

    loaders.insert(user_by_id_loader);

    loaders.insert(controller_by_id_loader);
    loaders.insert(controllers_by_creator_id_loader);
    loaders.insert(controllers_by_updater_id_loader);
    loaders.insert(controllers_by_facility_id_loader);
    loaders.insert(controllers_by_function_id_loader);
    loaders.insert(controllers_by_manufacturer_id_loader);

    loaders.insert(controller_function_by_id_loader);
    loaders.insert(controller_functions_by_creator_id_loader);
    loaders.insert(controller_functions_by_updater_id_loader);

    loaders.insert(controller_manufacturer_by_id_loader);
    loaders.insert(controller_manufacturers_by_creator_id_loader);
    loaders.insert(controller_manufacturers_by_updater_id_loader);

    loaders.insert(controller_change_by_id_loader);
    loaders.insert(controller_changes_by_controller_id_loader);
    loaders.insert(controller_changes_by_creator_id_loader);
    loaders.insert(controller_changes_by_updater_id_loader);

    loaders.insert(facility_by_id_loader);
    loaders.insert(facilities_by_creator_id_loader);
    loaders.insert(facilities_by_updater_id_loader);

    loaders
}
