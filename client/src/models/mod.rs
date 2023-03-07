pub mod mutations;
pub mod queries;

/// `UUID` is a custom scalar type defined in schema, so we have to provide matching Rust type.
pub type UUID = String;
