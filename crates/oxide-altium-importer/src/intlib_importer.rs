//! Altium Integrated Library (.IntLib) extractor and unified importer.

use oxide_library::primitive::footprint::Footprint;
use oxide_library::primitive::symbol::Symbol as LibSymbol;

use crate::cfb::CfbContainer;
use crate::error::AltiumImportError;
use crate::pcblib_importer::parse_footprints_from_records;
use crate::record::parse_record_stream;
use crate::schlib_importer::parse_symbols_from_records;

/// Extracted components from an Altium `.IntLib` container.
#[derive(Debug, Clone, Default)]
pub struct ExtractedIntLib {
    pub symbols: Vec<LibSymbol>,
    pub footprints: Vec<Footprint>,
}

/// Import an Altium Integrated Library (.IntLib) byte slice.
pub fn import_intlib_bytes(bytes: &[u8]) -> Result<ExtractedIntLib, AltiumImportError> {
    let cfb = CfbContainer::parse(bytes)?;
    let mut extracted = ExtractedIntLib::default();

    for (name, stream_bytes) in &cfb.streams {
        if let Ok(records) = parse_record_stream(stream_bytes) {
            // Try parsing as symbol stream
            if let Ok(syms) = parse_symbols_from_records(&records) {
                for s in syms {
                    if !extracted.symbols.iter().any(|existing| existing.name == s.name) {
                        extracted.symbols.push(s);
                    }
                }
            }

            // Try parsing as footprint stream
            if let Ok(fps) = parse_footprints_from_records(&records) {
                for f in fps {
                    if !extracted.footprints.iter().any(|existing| existing.name == f.name) {
                        extracted.footprints.push(f);
                    }
                }
            }
        }
    }

    Ok(extracted)
}
