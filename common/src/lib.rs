use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

mod compressor;
mod month_methane_emission;
mod pneumatic_instrument;
mod site;
mod storage_tank;
mod user;

pub use compressor::*;
pub use month_methane_emission::*;
pub use pneumatic_instrument::*;
pub use site::*;
pub use storage_tank::*;
pub use user::*;

/// `FacilityType` is an externally defined enum inside schema, so we have to provide matching Rust type and `Display` trait implementation.
///
/// It is defined in common library so it can be used by both server and client.
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(type_name = "facility_type", rename_all = "UPPERCASE")
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FacilityType {
    Tm,
    Wt,
    Ct,
    Ds,
    Gs,
    Ms,
    Gp,
    If,
    Pl,
    Wp,
    Ws,
    Bt,
}

impl Display for FacilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FacilityType::Tm => write!(f, "TM"),
            FacilityType::Wt => write!(f, "WT"),
            FacilityType::Ct => write!(f, "CT"),
            FacilityType::Ds => write!(f, "DS"),
            FacilityType::Gs => write!(f, "GS"),
            FacilityType::Ms => write!(f, "MS"),
            FacilityType::Gp => write!(f, "GP"),
            FacilityType::If => write!(f, "IF"),
            FacilityType::Pl => write!(f, "PL"),
            FacilityType::Wp => write!(f, "WP"),
            FacilityType::Ws => write!(f, "WS"),
            FacilityType::Bt => write!(f, "BT"),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), derive(async_graphql::Enum))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum UpdateFieldVariant {
    // Pneumatic Instrument
    PneumaticInstrumentSiteId,
    PneumaticInstrumentType,
    PneumaticInstrumentManufacturerId,
    PneumaticInstrumentModel,
    PneumaticInstrumentSerialNumber,
    PneumaticInstrumentStartDate,
    PneumaticInstrumentEndDate,
    // Pneumatic Instrument Change
    PneumaticInstrumentChangePneumaticInstrumentId,
    PneumaticInstrumentChangeDate,
    PneumaticInstrumentChangeRate,
    // Pneumatic Instrument Controlled Characterization
    PneumaticInstrumentControlledCharacterizationPneumaticInstrumentId,
    PneumaticInstrumentControlledCharacterizationStartDate,
    PneumaticInstrumentControlledCharacterizationEndDate,
    PneumaticInstrumentControlledCharacterizationControlDevice,
    PneumaticInstrumentControlledCharacterizationComment,
    // Pneumatic Instrument Control Device Inactivity
    PneumaticInstrumentControlDeviceInactivityPneumaticInstrumentControlledCharacterizationId,
    PneumaticInstrumentControlDeviceInactivityStartDate,
    PneumaticInstrumentControlDeviceInactivityEndDate,
    PneumaticInstrumentControlDeviceInactivityReason,
    PneumaticInstrumentControlDeviceInactivityComment,
    // Pneumatic Instrument Month Hours
    PneumaticInstrumentMonthHoursPneumaticInstrumentId,
    PneumaticInstrumentMonthHoursMonth,
    PneumaticInstrumentMonthHoursHoursOn,
    // Pneumatic Instrument Month Methane Emission Override
    PneumaticInstrumentMonthMethaneEmissionOverridePneumaticInstrumentId,
    PneumaticInstrumentMonthMethaneEmissionOverrideMonth,
    PneumaticInstrumentMonthMethaneEmissionOverrideGasVolume,
    PneumaticInstrumentMonthMethaneEmissionOverrideComment,
    // Level Controller
    LevelControllerSiteId,
    LevelControllerManufacturerId,
    LevelControllerModel,
    LevelControllerSerialNumber,
    LevelControllerStartDate,
    LevelControllerEndDate,
    // Level Controller Actuation Frequency
    LevelControllerActuationFrequencyLevelControllerId,
    LevelControllerActuationFrequencyDate,
    LevelControllerActuationFrequencyActuationFrequency,
    // Level Controller Change
    LevelControllerChangeLevelControllerId,
    LevelControllerChangeDate,
    LevelControllerChangeRate,
    // Level Controller Controlled Characterization
    LevelControllerControlledCharacterizationLevelControllerId,
    LevelControllerControlledCharacterizationStartDate,
    LevelControllerControlledCharacterizationEndDate,
    LevelControllerControlledCharacterizationControlDevice,
    LevelControllerControlledCharacterizationComment,
    // Level Controller Control Device Inactivity
    LevelControllerControlDeviceInactivityLevelControllerControlledCharacterizationId,
    LevelControllerControlDeviceInactivityStartDate,
    LevelControllerControlDeviceInactivityEndDate,
    LevelControllerControlDeviceInactivityReason,
    LevelControllerControlDeviceInactivityComment,
    // Level Controller Month Hours
    LevelControllerMonthHoursLevelControllerId,
    LevelControllerMonthHoursMonth,
    LevelControllerMonthHoursHoursOn,
    // Level Controller Month Methane Emission Override
    LevelControllerMonthMethaneEmissionOverrideLevelControllerId,
    LevelControllerMonthMethaneEmissionOverrideMonth,
    LevelControllerMonthMethaneEmissionOverrideGasVolume,
    LevelControllerMonthMethaneEmissionOverrideComment,
    // Compressor
    CompressorSiteId,
    CompressorFdcRecId,
    CompressorType,
    CompressorName,
    CompressorSerialNumber,
    CompressorPower,
    CompressorThrowCount,
    CompressorInstallDate,
    CompressorRemoveDate,
}

// graphql_client cannot handle OneofObject. InputObject has to be used instead and care must be made to not pass wrong value type to update_field mutation on the client side.
// Leaving this enum and OneofObject trait implementation in `common` library in case of potential upgrades to graphql_client in the future.
#[cfg_attr(not(target_arch = "wasm32"), derive(async_graphql::OneofObject))]
#[derive(Clone, PartialEq, Debug)]
pub enum UpdateFieldValueEnum {
    StringValue(String),
    OptionStringValue(Option<String>),
    IntegerValue(i64),
    OptionIntegerValue(Option<i64>),
    FloatValue(f64),
    OptionFloatValue(Option<f64>),
    UuidValue(Uuid),
    OptionUuidValue(Option<Uuid>),
    NaiveDateValue(NaiveDate),
    OptionNaiveDateValue(Option<NaiveDate>),
    NaiveDateTimeValue(NaiveDateTime),
    OptionNaiveDateTimeValue(Option<NaiveDateTime>),
}

impl Display for UpdateFieldValueEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StringValue(s) => write!(f, "{}", s),
            Self::OptionStringValue(os) => {
                write!(f, "{}", os.as_ref().map_or_else(|| "", |s| s))
            }
            Self::IntegerValue(i) => write!(f, "{}", i),
            Self::OptionIntegerValue(oi) => write!(
                f,
                "{}",
                oi.as_ref()
                    .map_or_else(|| "".to_string(), |s| s.to_string())
            ),
            Self::FloatValue(fl) => write!(f, "{}", fl),
            Self::OptionFloatValue(of) => write!(
                f,
                "{}",
                of.as_ref()
                    .map_or_else(|| "".to_string(), |fl| fl.to_string())
            ),
            Self::UuidValue(u) => write!(f, "{}", u),
            Self::OptionUuidValue(ou) => write!(
                f,
                "{}",
                ou.as_ref()
                    .map_or_else(|| "".to_string(), |u| u.to_string())
            ),
            Self::NaiveDateValue(nd) => write!(f, "{}", nd),
            Self::OptionNaiveDateValue(ond) => {
                write!(
                    f,
                    "{}",
                    ond.as_ref()
                        .map_or_else(|| "".to_string(), |nd| nd.to_string())
                )
            }
            Self::NaiveDateTimeValue(ndt) => write!(f, "{}", ndt),
            Self::OptionNaiveDateTimeValue(ondt) => {
                write!(
                    f,
                    "{}",
                    ondt.as_ref()
                        .map_or_else(|| "".to_string(), |ndt| ndt.to_string())
                )
            }
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), derive(async_graphql::Enum))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DeleteEntryVariant {
    Facility,
    Site,
    DeviceManufacturer,
    SurveyEquipment,
    GasAnalysis,
    // Pneumatic Instrument
    PneumaticInstrument,
    PneumaticInstrumentChange,
    PneumaticInstrumentControlledCharacterization,
    PneumaticInstrumentControlDeviceInactivity,
    PneumaticInstrumentMonthHours,
    PneumaticInstrumentMonthMethaneEmissionOverride,
    // Level Controller
    LevelController,
    LevelControllerActuationFrequency,
    LevelControllerChange,
    LevelControllerControlledCharacterization,
    LevelControllerControlDeviceInactivity,
    LevelControllerMonthHours,
    LevelControllerMonthMethaneEmissionOverride,
    // Pneumatic Pump
    PneumaticPump,
    PneumaticPumpChange,
    PneumaticPumpControlledCharacterization,
    PneumaticPumpControlDeviceInactivity,
    PneumaticPumpMonthHours,
    PneumaticPumpMonthMethaneEmissionOverride,
    // Compressor Seal
    Compressor,
    CompressorSeal,
    CompressorSealTest,
    CompressorEmissionSurvey,
    CompressorControlledCharacterization,
    CompressorControlDeviceInactivity,
    CompressorMonthHours,
    CompressorSealMonthMethaneEmissionOverride,
    CompressorBlowdown,
    CompressorBlowdownOverride,
    // Storage Tank
    StorageTank,
    StorageTankChange,
    StorageTankEmissionSurvey,
    StorageTankControlledCharacterization,
    StorageTankControlDeviceInactivity,
    StorageTankGasInSolutionFactorCalculated,
    StorageTankMonthLiquidHydrocarbonEntering,
    StorageTankMonthMethaneEmissionOverride,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(async_graphql::Enum))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GetObjectVariant {
    PneumaticInstrumentByFacilityId,
    CompressorByFacilityId,
    StorageTankByFacilityId,
    PneumaticInstrumentChangeByPneumaticInstrumentId,
    PneumaticInstrumentMonthHoursByPneumaticInstrumentId,
    PneumaticInstrumentMonthMethaneEmissionOverrideByPneumaticInstrumentId,
    PneumaticInstrumentMonthMethaneEmissionByPneumaticInstrumentId,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(async_graphql::Enum))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum IdSelectionVariant {
    DeviceManufacturerId,
    UserId,
}
