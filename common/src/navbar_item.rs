use strum_macros::{Display, EnumIter};

#[derive(PartialEq, Clone, Copy, Debug, Display, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum Emitter {
    PneumaticInstruments,
    LevelControllers,
    PneumaticPumps,
    CompressorSeals,
    StorageTanks,
}

impl Emitter {
    pub fn to_pretty_name(&self) -> String {
        self.to_string()
            .split('_')
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    None => String::new(),
                    Some(lc) => lc.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Display)]
#[strum(serialize_all = "title_case")]
pub enum SidebarItem {
    PneumaticInstrumentEmissionRate,
    PneumaticInstrumentMonthHours,
    PneumaticInstrumentMonthMethaneEmissionOverride,
    PneumaticInstrumentMonthMethaneEmission,
}
