use crate::graphql::models::{
    pneumatic_device::Controller, Compressor, ControllerChange, ControllerMonthHours,
    ControllerMonthVent, ControllerMonthVentOverride, TankFarm,
};
use async_graphql::SimpleObject;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Debug)]
pub struct GetObject {
    pub controllers: Option<Vec<Controller>>,
    pub compressors: Option<Vec<Compressor>>,
    pub tank_farms: Option<Vec<TankFarm>>,
    pub controller_changes: Option<Vec<ControllerChange>>,
    pub controller_month_hours: Option<Vec<ControllerMonthHours>>,
    pub controller_month_vent_override: Option<Vec<ControllerMonthVentOverride>>,
    pub controller_month_vent: Option<Vec<ControllerMonthVent>>,
}

#[derive(SimpleObject, Debug, Clone, FromRow, PartialEq)]
pub struct IdSelection {
    pub id: Uuid,
    pub name: String,
}
