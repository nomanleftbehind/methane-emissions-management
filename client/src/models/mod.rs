pub mod mutations;
pub mod queries;
use serde::Deserialize;
use std::fmt::Display;

/// `UUID` is a custom scalar type defined in schema, so we have to provide matching Rust type.
pub type UUID = String;

/// `Role` is an externally defined enum inside schema, so we have to provide matching Rust type and `Display` trait implementation.
#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
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
            Role::Admin => write!(f, "ADMIN"),
            Role::Engineer => write!(f, "ENGINEER"),
            Role::Office => write!(f, "OFFICE"),
            Role::Operator => write!(f, "OPERATOR"),
            Role::Regulatory => write!(f, "REGULATORY"),
        }
    }
}
