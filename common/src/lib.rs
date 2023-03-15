use std::fmt::Display;

use serde::{Deserialize, Serialize};

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
