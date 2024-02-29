use strum_macros::{Display, EnumIter};

#[derive(PartialEq, Clone, Copy, Debug, Display, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum Emitter {
    PneumaticInstruments,
    LevelControllers,
    PneumaticPumps,
    CompressorSeals,
    StorageTanks,
}

#[derive(Copy, Clone, PartialEq, Debug, Display)]
#[strum(serialize_all = "title_case")]
pub enum SidebarItem {
    PneumaticInstrumentEmissionRate,
    PneumaticInstrumentMonthHours,
    PneumaticInstrumentMonthMethaneEmissionOverride,
    PneumaticInstrumentMonthMethaneEmission,
}
