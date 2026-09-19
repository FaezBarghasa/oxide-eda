//! Error types for Altium Designer file import operations.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AltiumImportError {
    #[error("I/O error reading Altium container: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid Compound File Binary (CFB) header or corrupted OLE structure")]
    InvalidCfbHeader,

    #[error("Stream '{0}' not found in Altium container")]
    StreamNotFound(String),

    #[error("Decompression failed for stream '{0}': {1}")]
    DecompressionFailed(String, String),

    #[error("Failed to parse Altium record stream: {0}")]
    ParseError(String),

    #[error("Unsupported or corrupted Altium document format: {0}")]
    UnsupportedFormat(String),
}
