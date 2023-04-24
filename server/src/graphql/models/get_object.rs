use crate::graphql::models::{
    compressor::Compressor,
    defined_vent_gas::tank::Tank,
    month_methane_emission::MonthMethaneEmission,
    pneumatic_device::{
        PneumaticDevice, PneumaticDeviceChange, PneumaticDeviceMonthHours,
        PneumaticDeviceMonthMethaneEmissionOverride,
    },
};
use async_graphql::SimpleObject;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Debug)]
pub struct GetObject {
    pub pneumatic_devices: Option<Vec<PneumaticDevice>>,
    pub compressors: Option<Vec<Compressor>>,
    pub tanks: Option<Vec<Tank>>,
    pub pneumatic_device_changes: Option<Vec<PneumaticDeviceChange>>,
    pub pneumatic_device_month_hours: Option<Vec<PneumaticDeviceMonthHours>>,
    pub pneumatic_device_month_methane_emission_overrides:
        Option<Vec<PneumaticDeviceMonthMethaneEmissionOverride>>,
    pub month_methane_emissions: Option<Vec<MonthMethaneEmission>>,
}

#[derive(SimpleObject, Debug, Clone, FromRow, PartialEq)]
pub struct IdSelection {
    pub id: Uuid,
    pub name: String,
}
