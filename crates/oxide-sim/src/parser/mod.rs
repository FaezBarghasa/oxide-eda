//! Result parsers for SPICE simulation outputs.

pub mod csdf;
pub mod raw;

pub use csdf::parse_csdf;
pub use raw::parse_spice_raw;
