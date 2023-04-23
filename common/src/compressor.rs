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
