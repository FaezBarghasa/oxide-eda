//! Automated Output Job (`.snxoutjob`) Release Engine for Oxide EDA.
//!
//! Orchestrates single-click multi-format manufacturing and release packages:
//! Gerbers, Excellon NC Drills, Pick-and-Place CPL, Bill of Materials, and Schematics.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use oxide_types::pcb::PcbBoard;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::assembly::{AssemblyError, PickAndPlaceExporter, PickAndPlaceOptions};
use crate::bom::{BomError, BomExporter, BomFormat, BomOptions};
use crate::drill::excellon::{DrillError, ExcellonExporter};
use crate::gerber::{GerberError, GerberExporter, GerberOptions};
use crate::{ExportContext, ExportError, Exporter};

#[derive(Debug, Error)]
pub enum OutJobError {
    #[error("Gerber generation error: {0}")]
    Gerber(#[from] GerberError),
    #[error("Drill generation error: {0}")]
    Drill(#[from] DrillError),
    #[error("Pick and Place generation error: {0}")]
    Assembly(#[from] AssemblyError),
    #[error("BOM generation error: {0}")]
    Bom(#[from] BomError),
    #[error("Export error: {0}")]
    Export(#[from] ExportError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Output Job definition describing all release artifacts to generate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputJobConfig {
    pub name: String,
    pub generate_gerber: bool,
    pub generate_drills: bool,
    pub generate_pick_and_place: bool,
    pub generate_bom: bool,
    pub generate_pdf: bool,
    pub target_folder: PathBuf,
}

impl Default for OutputJobConfig {
    fn default() -> Self {
        Self {
            name: "Default_Release".to_string(),
            generate_gerber: true,
            generate_drills: true,
            generate_pick_and_place: true,
            generate_bom: true,
            generate_pdf: true,
            target_folder: PathBuf::from("dist/manufacturing"),
        }
    }
}

/// In-memory release artifact package.
#[derive(Debug, Clone, Default)]
pub struct ReleasePackage {
    pub files: HashMap<String, Vec<u8>>,
}

impl ReleasePackage {
    pub fn add_text_file(&mut self, filename: impl Into<String>, content: &str) {
        self.files
            .insert(filename.into(), content.as_bytes().to_vec());
    }

    pub fn add_binary_file(&mut self, filename: impl Into<String>, content: Vec<u8>) {
        self.files.insert(filename.into(), content);
    }

    /// Write all generated files to a destination directory.
    pub fn write_to_disk(&self, out_dir: &Path) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(out_dir)?;
        for (filename, data) in &self.files {
            let path = out_dir.join(filename);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&path, data)?;
        }
        Ok(())
    }
}

pub struct OutputJobRunner;

impl OutputJobRunner {
    /// Execute an output job and assemble the complete release package.
    pub fn run(
        config: &OutputJobConfig,
        ctx: &ExportContext,
        board: &PcbBoard,
    ) -> Result<ReleasePackage, OutJobError> {
        let mut pkg = ReleasePackage::default();

        // 1. Gerber Exporter
        if config.generate_gerber {
            let gerber_exp = GerberExporter::new(GerberOptions::default());
            let gerber_layers = gerber_exp.export_board(board)?;
            for layer in gerber_layers {
                pkg.add_text_file(format!("gerber/{}", layer.filename), &layer.content);
            }
        }

        // 2. Excellon NC Drill Exporter
        if config.generate_drills {
            let drill_exp = ExcellonExporter::new();
            let drill_files = drill_exp.export_board(board)?;
            for file in drill_files {
                pkg.add_text_file(format!("drill/{}", file.filename), &file.content);
            }
        }

        // 3. Pick and Place Exporter
        if config.generate_pick_and_place {
            let pnp_exp = PickAndPlaceExporter::new(PickAndPlaceOptions::default());
            let pnp_content = pnp_exp.export(board)?;
            pkg.add_text_file("assembly/pick_and_place.csv", &pnp_content);
        }

        // 4. Bill of Materials (BOM)
        if config.generate_bom {
            let bom_exp = BomExporter;
            let bom_opts = BomOptions {
                format: BomFormat::Csv,
                ..Default::default()
            };
            if let Ok(bom_output) = bom_exp.export(ctx, &bom_opts)
                && let Ok(content_str) = String::from_utf8(bom_output.bytes)
            {
                pkg.add_text_file("bom/bill_of_materials.csv", &content_str);
            }
        }

        // 5. Release Manifest
        let manifest = format!(
            "Oxide EDA Release Package: {}\nGenerated: {}\nTotal Files: {}\n",
            config.name,
            chrono::Utc::now().to_rfc3339(),
            pkg.files.len() + 1
        );
        pkg.add_text_file("MANIFEST.txt", &manifest);

        Ok(pkg)
    }
}
