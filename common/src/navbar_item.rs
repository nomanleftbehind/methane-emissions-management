use strum_macros::{Display, EnumIter};

#[derive(PartialEq, Clone, Copy, Debug, Display, EnumIter)]
#[strum(serialize_all = "title_case")]
pub enum Emitter {
    PneumaticInstrument,
    #[strum(serialize = "Non-Level Controller")]
    NonLevelController,
    PneumaticPump,
    CompressorSeal,
    StorageTank,
}

#[derive(Copy, Clone, PartialEq, Debug, Display)]
#[strum(serialize_all = "title_case")]
pub enum SidebarItem {
    PneumaticInstrumentChange,
    PneumaticInstrumentMonthHours,
    PneumaticInstrumentMonthMethaneEmissionOverride,
    PneumaticInstrumentMonthMethaneEmission,
}
