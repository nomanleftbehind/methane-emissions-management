use crate::models::queries::get_object::get_object::GetObjectVariant::{
    self, Other, COMPRESSOR_BY_FACILITY_ID, CONTROLLER_BY_FACILITY_ID,
    CONTROLLER_CHANGE_BY_CONTROLLER_ID, CONTROLLER_MONTH_HOURS_BY_CONTROLLER_ID,
    CONTROLLER_MONTH_VENT_BY_CONTROLLER_ID, CONTROLLER_MONTH_VENT_OVERRIDE_BY_CONTROLLER_ID,
    TANK_FARM_BY_FACILITY_ID,
};
use std::fmt::Display;

pub mod data;
pub mod delete_entry;
pub mod emitter_navbar;
pub mod emitters_window;
pub mod entry;
mod expand_data;
mod expand_svg;
pub mod id_selection;

#[derive(PartialEq, Clone, Copy)]
pub enum Emitter {
    Controller,
    Compressor,
    TankFarm,
}

impl Display for Emitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Emitter::Controller => write!(f, "Controllers"),
            Emitter::Compressor => write!(f, "Compressors"),
            Emitter::TankFarm => write!(f, "Tank Farms"),
        }
    }
}
impl Display for GetObjectVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CONTROLLER_BY_FACILITY_ID => write!(f, "Controllers"),
            COMPRESSOR_BY_FACILITY_ID => write!(f, "Compressors"),
            TANK_FARM_BY_FACILITY_ID => write!(f, "Tank Farms"),
            CONTROLLER_CHANGE_BY_CONTROLLER_ID => write!(f, "Controller Changes"),
            CONTROLLER_MONTH_HOURS_BY_CONTROLLER_ID => write!(f, "Controller Month Hours"),
            CONTROLLER_MONTH_VENT_OVERRIDE_BY_CONTROLLER_ID => {
                write!(f, "Controller Month Vent Override")
            }
            CONTROLLER_MONTH_VENT_BY_CONTROLLER_ID => write!(f, "Controller Month Vent"),
            Other(s) => write!(f, "Other: {}", s),
        }
    }
}
