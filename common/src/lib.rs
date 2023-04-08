use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

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

/// `Role` is an externally defined enum inside schema, so we have to provide matching Rust type and `Display` trait implementation.
///
/// It is defined in common library so it can be used by both server and client.
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(type_name = "user_role", rename_all = "UPPERCASE")
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Admin,
    Engineer,
    Regulatory,
    Office,
    Operator,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, " Admin"),
            Role::Engineer => write!(f, " Engineer"),
            Role::Regulatory => write!(f, " Regulatory"),
            Role::Office => write!(f, " Office"),
            Role::Operator => write!(f, " Operator"),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), derive(async_graphql::Enum))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum UpdateFieldVariant {
    ControllerFdcRecId,
    ControllerManufacturerId,
    ControllerModel,
    ControllerSerialNumber,
    ControllerApplicationId,
    ControllerFacilityId,
    CompressorFdcRecId,
    CompressorFacilityId,
    CompressorName,
    CompressorSerialNumber,
    CompressorInstallDate,
    CompressorRemoveDate,
    ControllerChangeId,
    ControllerChangeDate,
    ControllerChangeRate,
    ControllerMonthHoursControllerId,
    ControllerMonthHoursMonth,
    ControllerMonthHoursHoursOn,
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
    Controller,
    Compressor,
    TankFarm,
    ControllerChange,
    ControllerMonthHours,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(async_graphql::Enum))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GetObjectVariant {
    ControllerByFacilityId,
    CompressorByFacilityId,
    TankFarmByFacilityId,
    ControllerChangeByControllerId,
    ControllerMonthHoursByControllerId,
}
