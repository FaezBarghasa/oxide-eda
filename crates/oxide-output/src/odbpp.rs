//! ODB++ v8.1 Manufacturing Package Exporter.
//!
//! Generates hierarchical ODB++ directory structures and job archives for automated CAM tooling.

use oxide_types::pcb::PcbBoard;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum OdbError {
    #[error("ODB++ packaging error: {0}")]
    Pack(String),
}

#[derive(Debug, Clone)]
pub struct OdbFileEntry {
    pub relative_path: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct OdbOutputPackage {
    pub files: Vec<OdbFileEntry>,
}

/// Generates ODB++ v8.1 directory file entries for a PCB design.
pub fn export_odbpp_package(board: &PcbBoard) -> Result<OdbOutputPackage, OdbError> {
    let mut files = Vec::new();

    // 1. Matrix file
    let mut matrix = String::from("STEP { NAME=step }\nLAYER {\n");
    for (i, layer) in board.layers.iter().enumerate() {
        matrix.push_str(&format!(
            "  NAME={} TYPE={} CONTEXT=BOARD ROW={}\n",
            layer.name.replace(' ', "_"),
            layer.layer_type.to_uppercase(),
            i + 1
        ));
    }
    matrix.push_str("}\n");
    files.push(OdbFileEntry {
        relative_path: "matrix/matrix".to_string(),
        content: matrix,
    });

    // 2. Netlist file
    let mut netlist = String::from("$NETS\n");
    for net in &board.nets {
        netlist.push_str(&format!("  {} {}\n", net.number, net.name));
    }
    files.push(OdbFileEntry {
        relative_path: "steps/step/netlists/cadnet/netlist".to_string(),
        content: netlist,
    });

    // 3. Components / Components Top file
    let mut comp_top = String::from("# ODB++ Components Top\n");
    for fp in &board.footprints {
        if fp.layer.contains("Top") {
            comp_top.push_str(&format!(
                "CMP {} {} {:.4} {:.4} {:.2} P {}\n",
                fp.reference, fp.footprint_id, fp.position.x, fp.position.y, fp.rotation, fp.value
            ));
        }
    }
    files.push(OdbFileEntry {
        relative_path: "steps/step/layers/comp_+_top/components".to_string(),
        content: comp_top,
    });

    Ok(OdbOutputPackage { files })
}
