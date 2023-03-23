pub mod mutations;
pub mod queries;

/// `UUID` is a custom scalar type defined in schema, so we have to provide matching Rust type.
pub type UUID = uuid::Uuid;

/// `NaiveDate` is a custom scalar type defined in schema, so we have to provide matching Rust type.
pub type NaiveDate = chrono::NaiveDate;

/// `NaiveDateTime` is a custom scalar type defined in schema, so we have to provide matching Rust type.
pub type NaiveDateTime = chrono::NaiveDateTime;
