//! Result parsers for SPICE simulation outputs.

pub mod raw;
pub mod csdf;

pub use raw::parse_spice_raw;
pub use csdf::parse_csdf;
