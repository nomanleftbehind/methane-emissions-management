use super::pneumatic_device::non_level_controller::{
    NonLevelController, NonLevelControllerChange, NonLevelControllerMonthHours,
    NonLevelControllerMonthMethaneEmissionOverride,
};
use crate::graphql::models::{
    compressor::Compressor, defined_vent_gas::tank::Tank,
    month_methane_emission::MonthMethaneEmission,
};
use async_graphql::SimpleObject;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Debug)]
pub struct GetObject {
    pub non_level_controllers: Option<Vec<NonLevelController>>,
    pub compressors: Option<Vec<Compressor>>,
    pub tanks: Option<Vec<Tank>>,
    pub non_level_controller_changes: Option<Vec<NonLevelControllerChange>>,
    pub non_level_controller_month_hours: Option<Vec<NonLevelControllerMonthHours>>,
    pub non_level_controller_month_methane_emission_overrides:
        Option<Vec<NonLevelControllerMonthMethaneEmissionOverride>>,
    pub month_methane_emissions: Option<Vec<MonthMethaneEmission>>,
}

#[derive(SimpleObject, Debug, Clone, FromRow, PartialEq)]
pub struct IdSelection {
    pub id: Uuid,
    pub name: String,
}
