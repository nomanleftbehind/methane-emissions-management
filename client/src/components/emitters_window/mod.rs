use std::fmt::Display;

pub mod compressors;
pub mod controllers;
pub mod emitter_primary_navbar;
pub mod emitter_secondary_navbar;
pub mod emitters_window;

#[derive(PartialEq, Clone, Copy)]
pub enum Emitter {
    Controller(ControllerSubData),
    Compressor(CompressorSubData),
    TankFarm(TankFarmSubData),
}

#[derive(PartialEq, Clone, Copy)]
pub enum ControllerSubData {
    Controller,
    ControllerChange,
    ControllerMonthHours,
    ControllerMonthVent,
}

#[derive(PartialEq, Clone, Copy)]
pub enum CompressorSubData {
    Compressor,
    CompressorChange,
    CompressorMonthHours,
    CompressorMonthVent,
}

#[derive(PartialEq, Clone, Copy)]
pub enum TankFarmSubData {
    TankFarm,
    TankFarmChange,
    TankFarmMonthHours,
    TankFarmMonthVent,
}

impl PartialEq<ControllerSubData> for Emitter {
    fn eq(&self, other: &ControllerSubData) -> bool {
        match (self, other) {
            (Emitter::Controller(csd1), csd2) => csd1 == csd2,
            _ => false,
        }
    }
}

impl PartialEq<CompressorSubData> for Emitter {
    fn eq(&self, other: &CompressorSubData) -> bool {
        match (self, other) {
            (Emitter::Compressor(csd1), csd2) => csd1 == csd2,
            _ => false,
        }
    }
}

impl PartialEq<TankFarmSubData> for Emitter {
    fn eq(&self, other: &TankFarmSubData) -> bool {
        match (self, other) {
            (Emitter::TankFarm(tfsd1), tfsd2) => tfsd1 == tfsd2,
            _ => false,
        }
    }
}

impl Display for Emitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Emitter::Controller(_) => write!(f, "Controllers"),
            Emitter::Compressor(_) => write!(f, "Compressors"),
            Emitter::TankFarm(_) => write!(f, "Tank Farms"),
        }
    }
}

impl Display for ControllerSubData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControllerSubData::Controller => write!(f, "Controllers"),
            ControllerSubData::ControllerChange => write!(f, "Controller Changes"),
            ControllerSubData::ControllerMonthHours => write!(f, "Controller Month Hours"),
            ControllerSubData::ControllerMonthVent => write!(f, "Controller Month Vent"),
        }
    }
}

impl Display for CompressorSubData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompressorSubData::Compressor => write!(f, "Compressors"),
            CompressorSubData::CompressorChange => write!(f, "Compressor Changes"),
            CompressorSubData::CompressorMonthHours => write!(f, "Compressor Month Hours"),
            CompressorSubData::CompressorMonthVent => write!(f, "Compressor Month Vent"),
        }
    }
}

impl Display for TankFarmSubData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TankFarmSubData::TankFarm => write!(f, "Tank Farms"),
            TankFarmSubData::TankFarmChange => write!(f, "Tank Farm Changes"),
            TankFarmSubData::TankFarmMonthHours => write!(f, "Tank Farm Month Hours"),
            TankFarmSubData::TankFarmMonthVent => write!(f, "Tank Farm Month Vent"),
        }
    }
}
