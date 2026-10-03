use std::collections::HashSet;
use thiserror::Error;

use crate::primitive::footprint::Footprint;
use crate::primitive::symbol::Symbol;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum GateVerificationError {
    #[error(
        "GATE-PIN-01 Violation: Pin/Pad count mismatch. Symbol has {symbol_pins} pins, Footprint has {footprint_pads} pads"
    )]
    PinPadCountMismatch {
        symbol_pins: usize,
        footprint_pads: usize,
    },
    #[error("GATE-PIN-01 Violation: Missing pad for symbol pin '{0}'")]
    MissingPad(String),
    #[error("GATE-PIN-01 Violation: Orphan pad '{0}' without corresponding symbol pin")]
    OrphanPad(String),
    #[error("GATE-PIN-01 Violation: Duplicate symbol pin number '{0}'")]
    DuplicatePin(String),
    #[error("GATE-PIN-01 Violation: Duplicate footprint pad number '{0}'")]
    DuplicatePad(String),
    #[error(
        "GATE-IPC-02 Violation: Minimum clearance between pads {pad_a} and {pad_b} is {clearance_mm:.4} mm (required >= {required_mm:.4} mm)"
    )]
    InsufficientClearance {
        pad_a: String,
        pad_b: String,
        clearance_mm: f64,
        required_mm: f64,
    },
    #[error(
        "GATE-STEP-03 Violation: 3D body height {height_mm:.4} mm is non-positive or coplanarity offset {offset_z_mm:.4} mm exceeds tolerance"
    )]
    Invalid3DBody { height_mm: f32, offset_z_mm: f32 },
}

/// Senior QA Verification Engine enforcing Anti-Hallucination component gates.
pub struct VerificationEngine;

impl VerificationEngine {
    /// Validates GATE-PIN-01: Strict bijection between Symbol pins and Footprint pads.
    pub fn verify_gate_pin_01(
        symbol: &Symbol,
        footprint: &Footprint,
    ) -> Result<(), GateVerificationError> {
        let mut sym_pins = HashSet::new();
        for pin in &symbol.pins {
            if !sym_pins.insert(pin.number.clone()) {
                return Err(GateVerificationError::DuplicatePin(pin.number.clone()));
            }
        }

        let mut fp_pads = HashSet::new();
        for pad in &footprint.pads {
            if !fp_pads.insert(pad.number.clone()) {
                return Err(GateVerificationError::DuplicatePad(pad.number.clone()));
            }
        }

        // Check bijection
        for pin in &sym_pins {
            if !fp_pads.contains(pin) {
                return Err(GateVerificationError::MissingPad(pin.clone()));
            }
        }

        for pad in &fp_pads {
            if !sym_pins.contains(pad) {
                return Err(GateVerificationError::OrphanPad(pad.clone()));
            }
        }

        Ok(())
    }

    /// Validates GATE-IPC-02: Minimum copper clearance >= 0.10 mm.
    pub fn verify_gate_ipc_02(
        footprint: &Footprint,
        min_clearance_mm: f64,
    ) -> Result<(), GateVerificationError> {
        let pads = &footprint.pads;
        for i in 0..pads.len() {
            for j in (i + 1)..pads.len() {
                let p1 = &pads[i];
                let p2 = &pads[j];

                // Approximate distance between rectangular pad centers minus half-extents
                let dx = (p1.position[0] - p2.position[0]).abs();
                let dy = (p1.position[1] - p2.position[1]).abs();

                let span_x = (p1.size[0] + p2.size[0]) / 2.0;
                let span_y = (p1.size[1] + p2.size[1]) / 2.0;

                let gap_x = dx - span_x;
                let gap_y = dy - span_y;

                let clearance = if gap_x > 0.0 && gap_y > 0.0 {
                    (gap_x * gap_x + gap_y * gap_y).sqrt()
                } else if gap_x > 0.0 {
                    gap_x
                } else if gap_y > 0.0 {
                    gap_y
                } else {
                    0.0 // Overlap
                };

                if clearance < min_clearance_mm {
                    return Err(GateVerificationError::InsufficientClearance {
                        pad_a: p1.number.clone(),
                        pad_b: p2.number.clone(),
                        clearance_mm: clearance,
                        required_mm: min_clearance_mm,
                    });
                }
            }
        }
        Ok(())
    }

    /// Validates GATE-STEP-03: 3D body sanity and coplanarity.
    pub fn verify_gate_step_03(footprint: &Footprint) -> Result<(), GateVerificationError> {
        let body = &footprint.body_3d;
        if body.height_mm <= 0.0 || body.offset_z_mm.abs() > 0.05 {
            return Err(GateVerificationError::Invalid3DBody {
                height_mm: body.height_mm,
                offset_z_mm: body.offset_z_mm,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::footprint::{Footprint, LayerId, Pad, PadKind, PadShape};
    use crate::primitive::symbol::{Symbol, SymbolPin};

    #[test]
    fn gate_pin_01_accepts_bijective_match() {
        let mut sym = Symbol::empty("U1");
        sym.pins.push(SymbolPin::new("1", "IN"));
        sym.pins.push(SymbolPin::new("2", "OUT"));

        let mut fp = Footprint::empty("SOIC-2");
        fp.pads.push(Pad {
            number: "1".into(),
            kind: PadKind::Smd,
            shape: PadShape::Rect,
            size: [1.0, 1.0],
            position: [-1.0, 0.0],
            layers: vec![LayerId::new("F.Cu")],
            ..Pad::default()
        });
        fp.pads.push(Pad {
            number: "2".into(),
            kind: PadKind::Smd,
            shape: PadShape::Rect,
            size: [1.0, 1.0],
            position: [1.0, 0.0],
            layers: vec![LayerId::new("F.Cu")],
            ..Pad::default()
        });

        assert!(VerificationEngine::verify_gate_pin_01(&sym, &fp).is_ok());
    }

    #[test]
    fn gate_pin_01_rejects_missing_pad() {
        let mut sym = Symbol::empty("U1");
        sym.pins.push(SymbolPin::new("1", "IN"));
        sym.pins.push(SymbolPin::new("2", "OUT"));

        let mut fp = Footprint::empty("SOIC-1");
        fp.pads.push(Pad {
            number: "1".into(),
            kind: PadKind::Smd,
            shape: PadShape::Rect,
            size: [1.0, 1.0],
            position: [-1.0, 0.0],
            layers: vec![LayerId::new("F.Cu")],
            ..Pad::default()
        });

        let err = VerificationEngine::verify_gate_pin_01(&sym, &fp).unwrap_err();
        assert_eq!(err, GateVerificationError::MissingPad("2".into()));
    }
}
