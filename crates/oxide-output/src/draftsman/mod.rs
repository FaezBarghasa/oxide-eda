//! `draftsman` — Live Associative Production Engineering & Drafting Engine.
//!
//! Conforms to Master Technical Directive §4.5:
//! - Bidirectional Associative Model: Drawing views (Fabrication Views, Assembly Views, Drill Legends, Stackup Drawings)
//!   maintain direct live-linked DAG dependencies with the core PCB database.
//! - Standards Compliance: ASME Y14.5 and ISO 128 Geometric Dimensioning and Tolerancing (GD&T).
//! - Automated Table Synthesis: Real-time grouping of drill symbols, tolerances, and plating conditions.

pub mod drill_table;
pub mod gdt;

use serde::{Deserialize, Serialize};
use oxide_types::pcb::PcbBoard;

pub use drill_table::{DrillTable, DrillTableRow, HolePlating};
pub use gdt::{
    DatumReference, DimensionKind, FeatureControlFrame, GeometricCharacteristic, MaterialCondition,
};

/// Standard Sheet Formats for Engineering Drawings (ASME & ISO).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SheetSize {
    // ISO 216
    A4Landscape,
    A3Landscape,
    A2Landscape,
    A1Landscape,
    A0Landscape,
    // ANSI / ASME Y14.1
    AnsiA,
    AnsiB,
    AnsiC,
    AnsiD,
    AnsiE,
}

impl SheetSize {
    /// Returns (width_mm, height_mm).
    pub fn dimensions_mm(&self) -> (f64, f64) {
        match self {
            Self::A4Landscape => (297.0, 210.0),
            Self::A3Landscape => (420.0, 297.0),
            Self::A2Landscape => (594.0, 420.0),
            Self::A1Landscape => (841.0, 594.0),
            Self::A0Landscape => (1189.0, 841.0),
            Self::AnsiA => (279.4, 215.9),
            Self::AnsiB => (431.8, 279.4),
            Self::AnsiC => (558.8, 431.8),
            Self::AnsiD => (863.6, 558.8),
            Self::AnsiE => (1117.6, 863.6),
        }
    }
}

/// A Drawing View placed on a Draftsman sheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DrawingView {
    /// Board Fabrication View showing board outline, holes, routing cutouts, and dimensions
    FabricationView {
        title: String,
        position_mm: [f64; 2],
        scale: f64,
        show_dimensions: bool,
    },
    /// Board Assembly View showing component outlines, designators, and polarity markers
    AssemblyView {
        title: String,
        position_mm: [f64; 2],
        scale: f64,
        is_top_side: bool,
    },
    /// Fabrication Drill Table & Symbol Legend
    DrillLegend {
        position_mm: [f64; 2],
        table: DrillTable,
    },
    /// Layer Stack Legend showing copper, dielectric, core, and solder mask thicknesses
    LayerStackLegend {
        position_mm: [f64; 2],
    },
}

/// A single Draftsman drawing sheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftsmanSheet {
    pub sheet_number: usize,
    pub title: String,
    pub size: SheetSize,
    pub views: Vec<DrawingView>,
    pub dimensions: Vec<DimensionKind>,
}

impl DraftsmanSheet {
    pub fn new(sheet_number: usize, title: &str, size: SheetSize) -> Self {
        Self {
            sheet_number,
            title: title.to_string(),
            size,
            views: Vec::new(),
            dimensions: Vec::new(),
        }
    }
}

/// Complete Draftsman Drawing Document (`.snxdraft`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftsmanDocument {
    pub project_name: String,
    pub revision: String,
    pub company_name: String,
    pub sheets: Vec<DraftsmanSheet>,
}

impl DraftsmanDocument {
    pub fn new(project_name: &str) -> Self {
        let mut doc = Self {
            project_name: project_name.to_string(),
            revision: "1.0".to_string(),
            company_name: "Oxide EDA Systems".to_string(),
            sheets: Vec::new(),
        };

        // Initialize with standard A3 fabrication & assembly sheets
        let mut sheet1 = DraftsmanSheet::new(1, "Fabrication & Drill Drawing", SheetSize::A3Landscape);
        sheet1.views.push(DrawingView::FabricationView {
            title: "BOARD FABRICATION VIEW".to_string(),
            position_mm: [150.0, 150.0],
            scale: 1.0,
            show_dimensions: true,
        });
        sheet1.views.push(DrawingView::DrillLegend {
            position_mm: [320.0, 20.0],
            table: DrillTable::default(),
        });
        doc.sheets.push(sheet1);

        let mut sheet2 = DraftsmanSheet::new(2, "Top Assembly Drawing", SheetSize::A3Landscape);
        sheet2.views.push(DrawingView::AssemblyView {
            title: "TOP SIDE ASSEMBLY VIEW".to_string(),
            position_mm: [210.0, 150.0],
            scale: 1.0,
            is_top_side: true,
        });
        doc.sheets.push(sheet2);

        doc
    }

    /// Live synchronization from updated PCB board layout.
    pub fn sync_with_board(&mut self, board: &PcbBoard) {
        let new_drill_table = DrillTable::from_board(board);

        for sheet in &mut self.sheets {
            for view in &mut sheet.views {
                if let DrawingView::DrillLegend { table, .. } = view {
                    *table = new_drill_table.clone();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draftsman_document_creation_and_sync() {
        let mut doc = DraftsmanDocument::new("Smart_Sensor_Node");
        assert_eq!(doc.sheets.len(), 2);
        assert_eq!(doc.sheets[0].views.len(), 2);

        let mut board = PcbBoard::default();
        board.vias.push(oxide_types::pcb::Via {
            id: uuid::Uuid::new_v4(),
            position: oxide_types::pcb::Point::new(0, 0),
            net_id: None,
            size_nm: 600_000,
            drill_nm: 300_000,
            via_type: oxide_types::pcb::ViaType::Through,
            via_span: None,
        });

        doc.sync_with_board(&board);

        if let DrawingView::DrillLegend { table, .. } = &doc.sheets[0].views[1] {
            assert_eq!(table.total_hole_count, 1);
            assert_eq!(table.rows.len(), 1);
            assert_eq!(table.rows[0].diameter_mm, 0.3);
        } else {
            panic!("Expected DrillLegend view");
        }
    }
}
