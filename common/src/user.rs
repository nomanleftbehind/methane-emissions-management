use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
