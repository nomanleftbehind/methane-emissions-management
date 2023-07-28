use strum_macros::{Display, EnumIter};

#[derive(PartialEq, Clone, Copy, Debug, Display, EnumIter)]
#[strum(serialize_all = "title_case")]
pub enum Emitter {
    PneumaticInstrument,
    LevelController,
    PneumaticPump,
    CompressorSeal,
    StorageTank,
}

#[derive(Copy, Clone, PartialEq, Debug, Display)]
#[strum(serialize_all = "title_case")]
pub enum SidebarItem {
    PneumaticInstrumentEmissionRate,
    PneumaticInstrumentMonthHours,
    PneumaticInstrumentMonthMethaneEmissionOverride,
    PneumaticInstrumentMonthMethaneEmission,
}
