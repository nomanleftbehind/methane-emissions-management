pub mod compressor_blowdown_loader;
pub mod compressor_change_loader;
pub mod compressor_loader;
pub mod compressor_month_hours_loader;
pub mod compressor_month_vent_loader;
pub mod controller_application_loader;
pub mod controller_change_loader;
pub mod controller_loader;
pub mod controller_manufacturer_loader;
pub mod controller_month_hours_loader;
pub mod controller_month_vent_loader;
pub mod facility_loader;
pub mod gas_analysis_calculated_param_loader;
pub mod gas_analysis_loader;
mod loader_registry;
pub mod tank_farm_change_loader;
pub mod tank_farm_loader;
pub mod tank_farm_month_oil_flow_loader;
pub mod tank_farm_vent_factor_loader;
pub mod user_loader;

pub use loader_registry::{get_loaders, LoaderRegistry};
