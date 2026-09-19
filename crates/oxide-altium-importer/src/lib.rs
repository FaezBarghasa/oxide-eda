//! Cleanroom pure-Rust Altium Designer (.SchDoc, .PcbDoc, .SchLib, .PcbLib, .IntLib) importer.
//!
//! # Overview
//! Reads and translates proprietary Altium OLE2/CFB binary streams into native
//! Oxide EDA types ([`SchematicSheet`], [`PcbBoard`], [`LibSymbol`], and [`Footprint`]).

pub mod cfb;
pub mod error;
pub mod intlib_importer;
pub mod pcb_importer;
pub mod pcblib_importer;
pub mod record;
pub mod sch_importer;
pub mod schlib_importer;

pub use cfb::CfbContainer;
pub use error::AltiumImportError;
pub use intlib_importer::{import_intlib_bytes, ExtractedIntLib};
pub use pcb_importer::import_pcbdoc_bytes;
pub use pcblib_importer::import_pcblib_bytes;
pub use record::{parse_record_stream, AltiumRecord};
pub use sch_importer::import_schdoc_bytes;
pub use schlib_importer::import_schlib_bytes;

use std::path::Path;

/// High-level file loader: detect format by extension and import.
pub enum AltiumImportResult {
    Schematic(oxide_types::schematic::SchematicSheet),
    Pcb(oxide_types::pcb::PcbBoard),
    SymbolLibrary(Vec<oxide_library::primitive::symbol::Symbol>),
    FootprintLibrary(Vec<oxide_library::primitive::footprint::Footprint>),
    IntegratedLibrary(ExtractedIntLib),
}

/// Import an Altium Designer file from a local filesystem path.
pub fn import_altium_file<P: AsRef<Path>>(path: P) -> Result<AltiumImportResult, AltiumImportError> {
    let p = path.as_ref();
    let bytes = std::fs::read(p)?;
    let ext = p
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "schdoc" => {
            let sheet = import_schdoc_bytes(&bytes)?;
            Ok(AltiumImportResult::Schematic(sheet))
        }
        "pcbdoc" => {
            let board = import_pcbdoc_bytes(&bytes)?;
            Ok(AltiumImportResult::Pcb(board))
        }
        "schlib" => {
            let syms = import_schlib_bytes(&bytes)?;
            Ok(AltiumImportResult::SymbolLibrary(syms))
        }
        "pcblib" => {
            let fps = import_pcblib_bytes(&bytes)?;
            Ok(AltiumImportResult::FootprintLibrary(fps))
        }
        "intlib" => {
            let intlib = import_intlib_bytes(&bytes)?;
            Ok(AltiumImportResult::IntegratedLibrary(intlib))
        }
        _ => Err(AltiumImportError::UnsupportedFormat(format!(
            "Unknown or unsupported Altium extension: .{ext}"
        ))),
    }
}
