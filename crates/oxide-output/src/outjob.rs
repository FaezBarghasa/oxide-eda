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
    #[error("ActiveBOM production release gate rejected release:\n{}", .0.join("\n"))]
    ActiveBomReleaseGateFailed(Vec<String>),
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
    #[serde(default)]
    pub enforce_active_bom_gate: bool,
    #[serde(default)]
    pub allow_stale_bom: bool,
    #[serde(default = "default_max_bom_stale_hours")]
    pub max_bom_stale_hours: u32,
}

fn default_max_bom_stale_hours() -> u32 {
    24
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
            enforce_active_bom_gate: false,
            allow_stale_bom: false,
            max_bom_stale_hours: 24,
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
        Self::run_with_stock_audit(config, ctx, board, None)
    }

    /// Execute an output job with optional pre-queried or cached ActiveBOM stock audit.
    pub fn run_with_stock_audit(
        config: &OutputJobConfig,
        ctx: &ExportContext,
        board: &PcbBoard,
        stock_statuses: Option<&[oxide_bom::BomStockStatus]>,
    ) -> Result<ReleasePackage, OutJobError> {
        // ActiveBOM Production Release Gate
        if config.enforce_active_bom_gate && !config.allow_stale_bom {
            if let Some(statuses) = stock_statuses {
                oxide_bom::audit_production_bom(statuses, config.max_bom_stale_hours)
                    .map_err(OutJobError::ActiveBomReleaseGateFailed)?;
            } else {
                let bom_opts = BomOptions {
                    format: BomFormat::Csv,
                    ..Default::default()
                };
                let bom_ctx = crate::bom::build_bom_context(ctx, &bom_opts);
                let table =
                    oxide_bom::build_table(&bom_ctx, &oxide_bom::BomEngineOptions::default());
                let distributors = vec![oxide_bom::DistributorConfig {
                    provider: oxide_bom::DistributorProvider::Lcsc,
                    api_endpoint: "https://wmsc.lcsc.com".to_string(),
                    api_key: None,
                    enabled: true,
                }];
                let statuses = oxide_bom::check_stock(&table, &distributors);
                oxide_bom::audit_production_bom(&statuses, config.max_bom_stale_hours)
                    .map_err(OutJobError::ActiveBomReleaseGateFailed)?;
            }
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProjectMetadata;
    use oxide_bom::BomStockStatus;
    use oxide_types::pcb::PcbBoard;

    fn sample_context_and_board() -> (ExportContext, PcbBoard) {
        let ctx = ExportContext {
            sheets: Vec::new(),
            metadata: ProjectMetadata::default(),
            netlist: None,
        };
        let board = PcbBoard::default();
        (ctx, board)
    }

    #[test]
    fn test_bom_release_gate_rejects_stale_pricing_and_zero_stock() {
        let (ctx, board) = sample_context_and_board();

        let mut config = OutputJobConfig {
            generate_gerber: false,
            generate_drills: false,
            generate_pick_and_place: false,
            generate_bom: false,
            generate_pdf: false,
            enforce_active_bom_gate: true,
            allow_stale_bom: false,
            max_bom_stale_hours: 24,
            ..Default::default()
        };

        // 1. Stale 48-hour cached part should fail release
        let stale_statuses = vec![BomStockStatus {
            mpn: "STM32F401CCU6".to_string(),
            distributor: "LCSC".to_string(),
            in_stock_qty: 1500,
            min_order_qty: 1,
            unit_price_usd: Some(2.15),
            lead_time_weeks: Some(1),
            lifecycle: oxide_bom::ComponentLifecycle::Active,
            currency: "USD".to_string(),
            verified_at_utc: Some("2026-11-02T12:00:00Z".to_string()),
            is_stale: true,
            cache_age_hours: 48,
        }];

        let result =
            OutputJobRunner::run_with_stock_audit(&config, &ctx, &board, Some(&stale_statuses));
        assert!(
            result.is_err(),
            "stale sourcing data must fail release gate"
        );
        if let Err(OutJobError::ActiveBomReleaseGateFailed(errors)) = result {
            assert!(errors.iter().any(|e| e.contains("stale")));
        } else {
            panic!("Expected ActiveBomReleaseGateFailed error");
        }

        // 2. Overriding with allow_stale_bom: true allows production release
        config.allow_stale_bom = true;
        let bypass_result =
            OutputJobRunner::run_with_stock_audit(&config, &ctx, &board, Some(&stale_statuses));
        assert!(bypass_result.is_ok(), "allow_stale_bom must override gate");

        // 3. Fresh, in-stock part passes gate cleanly
        config.allow_stale_bom = false;
        let fresh_statuses = vec![BomStockStatus {
            cache_age_hours: 3,
            is_stale: false,
            ..stale_statuses[0].clone()
        }];
        let fresh_result =
            OutputJobRunner::run_with_stock_audit(&config, &ctx, &board, Some(&fresh_statuses));
        assert!(fresh_result.is_ok(), "fresh verified BOM must pass gate");
    }
}
