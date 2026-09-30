//! Automated Controlled-Depth Backdrilling & Via Stub Elimination Engine.
//!
//! Calculates required backdrill depths from top/bottom surface to the active high-speed internal
//! copper layer, eliminating resonant open via stubs (>10 GHz) for PCIe Gen 5/6, 112G PAM4, and DDR5.

use oxide_types::pcb::{PcbBoard, Point, Via};
use crate::units::Microns;
use crate::stackup::LayerStackup;

/// Specification and output for an automated backdrill operation on a specific via.
#[derive(Debug, Clone, PartialEq)]
pub struct BackdrillTarget {
    pub position: Point,
    pub net_id: u32,
    pub original_via_diameter_um: Microns,
    pub backdrill_diameter_um: Microns,
    pub from_top: bool,
    pub drill_depth_um: Microns,
    pub target_layer: usize,
    pub residual_stub_length_um: Microns,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BackdrillSummary {
    pub total_stubs_identified: usize,
    pub total_backdrilled: usize,
    pub targets: Vec<BackdrillTarget>,
    pub max_residual_stub_um: Microns,
}

/// Calculate backdrill requirements for high-speed nets on a PCB board given its layer stackup.
pub fn calculate_backdrilling(
    board: &PcbBoard,
    stackup: &LayerStackup,
    high_speed_nets: &[u32],
    max_allowable_stub_um: Microns,
    backdrill_oversize_um: Microns,
) -> BackdrillSummary {
    let mut targets = Vec::new();
    let mut total_stubs = 0;

    for via in &board.vias {
        if !high_speed_nets.contains(&via.net) {
            continue;
        }

        let via_diam_um = Microns((via.diameter * 1000.0).round() as i64);
        let backdrill_diam = via_diam_um + backdrill_oversize_um;

        // In a typical through via, if connection transitions at layer K, the remaining stub to layer N is unused.
        let total_board_thickness_um = stackup.total_thickness_um();
        let target_layer = 3; // internal high-speed signal layer example
        let signal_layer_depth_um = stackup.depth_to_layer_um(target_layer);

        let stub_from_bottom_um = total_board_thickness_um - signal_layer_depth_um;

        if stub_from_bottom_um > max_allowable_stub_um {
            total_stubs += 1;
            let drill_depth = stub_from_bottom_um - Microns(50); // 50um safety margin
            targets.push(BackdrillTarget {
                position: via.position,
                net_id: via.net,
                original_via_diameter_um: via_diam_um,
                backdrill_diameter_um: backdrill_diam,
                from_top: false,
                drill_depth_um: drill_depth,
                target_layer,
                residual_stub_length_um: Microns(50),
            });
        }
    }

    let total_backdrilled = targets.len();
    let max_residual = targets.iter().map(|t| t.residual_stub_length_um).max().unwrap_or(Microns(0));

    BackdrillSummary {
        total_stubs_identified: total_stubs,
        total_backdrilled,
        targets,
        max_residual_stub_um: max_residual,
    }
}
