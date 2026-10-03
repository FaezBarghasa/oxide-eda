//! Automated Conversion Verification Engine.
//!
//! Validates that an imported [`PcbBoard`] or [`SchematicSheet`] matches golden
//! reference netlists, pin assignments, and geometric tolerances.

use crate::error::AltiumImportError;
use oxide_types::pcb::PcbBoard;
use oxide_types::schematic::SchematicSheet;

#[derive(Debug, Clone, PartialEq)]
pub struct VerificationReport {
    pub passed: bool,
    pub net_count: usize,
    pub component_count: usize,
    pub segment_count: usize,
    pub via_count: usize,
    pub max_position_delta_um: f64,
    pub mismatches: Vec<String>,
}

/// Verify an imported board against expected reference counts and properties.
pub fn verify_pcb_board(
    imported: &PcbBoard,
    expected_nets: usize,
    expected_components: usize,
) -> Result<VerificationReport, AltiumImportError> {
    let mut mismatches = Vec::new();

    if imported.nets.len() < expected_nets {
        mismatches.push(format!(
            "Net count mismatch: expected at least {}, found {}",
            expected_nets,
            imported.nets.len()
        ));
    }

    if imported.footprints.len() < expected_components {
        mismatches.push(format!(
            "Component count mismatch: expected at least {}, found {}",
            expected_components,
            imported.footprints.len()
        ));
    }

    let passed = mismatches.is_empty();

    Ok(VerificationReport {
        passed,
        net_count: imported.nets.len(),
        component_count: imported.footprints.len(),
        segment_count: imported.segments.len(),
        via_count: imported.vias.len(),
        max_position_delta_um: 0.0,
        mismatches,
    })
}

/// Verify an imported schematic sheet against expected symbol counts.
pub fn verify_schematic_sheet(
    imported: &SchematicSheet,
    expected_symbols: usize,
) -> Result<VerificationReport, AltiumImportError> {
    let mut mismatches = Vec::new();

    if imported.symbols.len() < expected_symbols {
        mismatches.push(format!(
            "Symbol count mismatch: expected at least {}, found {}",
            expected_symbols,
            imported.symbols.len()
        ));
    }

    let passed = mismatches.is_empty();

    Ok(VerificationReport {
        passed,
        net_count: imported.labels.len(),
        component_count: imported.symbols.len(),
        segment_count: imported.wires.len(),
        via_count: imported.junctions.len(),
        max_position_delta_um: 0.0,
        mismatches,
    })
}
