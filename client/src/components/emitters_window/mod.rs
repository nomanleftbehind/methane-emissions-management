use std::fmt::Display;

pub mod compressors;
mod controller_data;
mod controller_row;
pub mod controllers;
pub mod delete_entry;
pub mod emitter_navbar;
pub mod emitters_window;
pub mod entry;
mod expand_data;
mod expand_svg;
pub mod tank_farms;

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
