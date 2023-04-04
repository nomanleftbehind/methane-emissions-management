use crate::graphql::models::{Compressor, Controller, ControllerChange, TankFarm};
use async_graphql::SimpleObject;

#[derive(SimpleObject, Debug)]
pub struct GetObject {
    pub controllers: Option<Vec<Controller>>,
    pub compressors: Option<Vec<Compressor>>,
    pub tank_farms: Option<Vec<TankFarm>>,
    pub controller_changes: Option<Vec<ControllerChange>>,
}