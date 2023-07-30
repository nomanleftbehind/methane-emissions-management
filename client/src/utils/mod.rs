pub mod common;
mod console_log;
pub mod error;
pub mod gen_style;
#[allow(non_camel_case_types)]
mod insert_form;

pub(crate) use console_log::console_log;
pub use insert_form::*;
