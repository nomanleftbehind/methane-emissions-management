use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

mod compressor;
mod manual_mutation;
mod month_methane_emission;
mod pneumatic_instrument;
mod site;
mod storage_tank;
mod user;

pub use compressor::*;
pub use manual_mutation::*;
pub use month_methane_emission::*;
pub use pneumatic_instrument::*;
pub use site::*;
pub use storage_tank::*;
pub use user::*;

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
    BoolValue(bool),
    OptionBoolValue(Option<bool>),
    FacilityTypeValue(FacilityType),
    OptionFacilityTypeValue(Option<FacilityType>),
    SiteTypeValue(SiteType),
    OptionSiteTypeValue(Option<SiteType>),
    PneumaticInstrumentTypeValue(PneumaticInstrumentType),
    OptionPneumaticInstrumentTypeValue(Option<PneumaticInstrumentType>),
    CompressorTypeValue(CompressorType),
    OptionCompressorTypeValue(Option<CompressorType>),
    ControlDeviceValue(ControlDevice),
    OptionControlDeviceValue(Option<ControlDevice>),
    ControlDeviceInactivityReasonValue(ControlDeviceInactivityReason),
    OptionControlDeviceInactivityReasonValue(Option<ControlDeviceInactivityReason>),
    SealTypeValue(SealType),
    OptionSealTypeValue(Option<SealType>),
    CompressorSealTestingPointValue(CompressorSealTestingPoint),
    OptionCompressorSealTestingPointValue(Option<CompressorSealTestingPoint>),
    CalculationMethodValue(CalculationMethod),
    OptionCalculationMethodValue(Option<CalculationMethod>),
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
            Self::BoolValue(b) => write!(f, "{}", b),
            Self::OptionBoolValue(ob) => write!(
                f,
                "{}",
                ob.as_ref()
                    .map_or_else(|| "".to_string(), |s| s.to_string())
            ),
            Self::FacilityTypeValue(ft) => write!(f, "{}", ft),
            Self::OptionFacilityTypeValue(oft) => write!(
                f,
                "{}",
                oft.as_ref()
                    .map_or_else(|| "".to_string(), |s| s.to_string())
            ),
            Self::SiteTypeValue(st) => write!(f, "{}", st),
            Self::OptionSiteTypeValue(ost) => write!(
                f,
                "{}",
                ost.as_ref()
                    .map_or_else(|| "".to_string(), |s| s.to_string())
            ),
            Self::PneumaticInstrumentTypeValue(pit) => write!(f, "{}", pit),
            Self::OptionPneumaticInstrumentTypeValue(opit) => write!(
                f,
                "{}",
                opit.as_ref()
                    .map_or_else(|| "".to_string(), |s| s.to_string())
            ),
            Self::CompressorTypeValue(ct) => write!(f, "{}", ct),
            Self::OptionCompressorTypeValue(oct) => write!(
                f,
                "{}",
                oct.as_ref()
                    .map_or_else(|| "".to_string(), |s| s.to_string())
            ),
            Self::ControlDeviceValue(cd) => write!(f, "{}", cd),
            Self::OptionControlDeviceValue(ocd) => write!(
                f,
                "{}",
                ocd.as_ref()
                    .map_or_else(|| "".to_string(), |s| s.to_string())
            ),
            Self::ControlDeviceInactivityReasonValue(cdir) => write!(f, "{}", cdir),
            Self::OptionControlDeviceInactivityReasonValue(ocdir) => write!(
                f,
                "{}",
                ocdir
                    .as_ref()
                    .map_or_else(|| "".to_string(), |s| s.to_string())
            ),
            Self::SealTypeValue(st) => write!(f, "{}", st),
            Self::OptionSealTypeValue(ost) => write!(
                f,
                "{}",
                ost.as_ref()
                    .map_or_else(|| "".to_string(), |s| s.to_string())
            ),
            Self::CompressorSealTestingPointValue(cstp) => write!(f, "{}", cstp),
            Self::OptionCompressorSealTestingPointValue(ocstp) => write!(
                f,
                "{}",
                ocstp
                    .as_ref()
                    .map_or_else(|| "".to_string(), |s| s.to_string())
            ),
            Self::CalculationMethodValue(cm) => write!(f, "{}", cm),
            Self::OptionCalculationMethodValue(ocm) => write!(
                f,
                "{}",
                ocm.as_ref()
                    .map_or_else(|| "".to_string(), |s| s.to_string())
            ),
        }
    }
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
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize)]
pub enum IdSelectionVariant {
    DeviceManufacturerId,
    UserId,
}
