use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// `SealType` is an externally defined enum inside schema, so we have to provide matching Rust type and `Display` trait implementation.
///
/// It is defined in common library so it can be used by both server and client.
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(type_name = "seal_type", rename_all = "SCREAMING_SNAKE_CASE")
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SealType {
    Rodpacking,
    Dry,
    Wet,
}

impl Display for SealType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SealType::Rodpacking => write!(f, "Rodpacking"),
            SealType::Dry => write!(f, "Dry"),
            SealType::Wet => write!(f, "Wet"),
        }
    }
}

/// Type representing possible compressor seal testing points from AER Directive 060 section [`8.6.2.1`](https://static.aer.ca/prd/documents/directives/Directive060.pdf#page=76), AER Manual 015 section [`4.5.1`](https://static.aer.ca/prd/documents/manuals/Manual015.pdf#page=41) and [`4.5.2`](https://static.aer.ca/prd/documents/manuals/Manual015.pdf#page=42).
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(
        type_name = "compressor_seal_testing_point",
        rename_all = "SCREAMING_SNAKE_CASE"
    )
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CompressorSealTestingPoint {
    PistonRodPacking,
    DistancePiece,
    Crankcase,
    DriveShaftAndCompressorCaseInterface,
}

impl Display for CompressorSealTestingPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompressorSealTestingPoint::PistonRodPacking => write!(f, "Piston-Rod-Packing"),
            CompressorSealTestingPoint::DistancePiece => write!(f, "Distance-Piece"),
            CompressorSealTestingPoint::Crankcase => write!(f, "Crankcase"),
            CompressorSealTestingPoint::DriveShaftAndCompressorCaseInterface => {
                write!(f, "Drive Shaft and Compressor Case Interface")
            }
        }
    }
}

/// `CompressorType` is an externally defined enum inside schema, so we have to provide matching Rust type and `Display` trait implementation.
///
/// It is defined in common library so it can be used by both server and client.
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(type_name = "compressor_type", rename_all = "SCREAMING_SNAKE_CASE")
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CompressorType {
    Reciprocating,
    Centrifugal,
    Screw,
    Scroll,
}

impl Display for CompressorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompressorType::Reciprocating => write!(f, "Reciprocating"),
            CompressorType::Centrifugal => write!(f, "Centrifugal"),
            CompressorType::Screw => write!(f, "Screw"),
            CompressorType::Scroll => write!(f, "Scroll"),
        }
    }
}

/// Type used to abstract [`AER Manual 015 section 1.1.2`](https://static.aer.ca/prd/documents/manuals/Manual015.pdf#page=10).
///
/// `ControlledCharacterization` is a database defined enum, so we have to provide matching Rust type and `Display` trait implementation.
///
/// It is defined in common library so it can be used by both server and client.
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(
        type_name = "controlled_characterization",
        rename_all = "SCREAMING_SNAKE_CASE"
    )
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ControlledCharacterization {
    Controlled,
    Uncontrolled,
}

impl Display for ControlledCharacterization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlledCharacterization::Controlled => write!(f, "Controlled"),
            ControlledCharacterization::Uncontrolled => write!(f, "Uncontrolled"),
        }
    }
}
