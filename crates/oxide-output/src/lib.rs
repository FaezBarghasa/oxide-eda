//! Output generation for Oxide — PDF, BOM, netlist.
//!
//! See `docs/internal/docs/OUTPUT_PLAN.md` for the v0.8 design.
//!
//! v0.8.0 ships: PDF, netlist, print preview, sheet templates, text substitution.
//! v0.8.1 ships: BOM (CSV / HTML / XLSX).

use std::path::PathBuf;

use oxide_types::schematic::SchematicSheet;
use thiserror::Error;

pub mod assembly;
pub mod bom;
pub mod draftsman;
pub mod drill;
mod expression;
pub mod gerber;
pub mod netlist;
pub mod outjob;
pub mod pdf;
pub mod preview;
pub mod substitution;
pub mod svg;
pub mod template;

pub use assembly::{AssemblyError, AssemblyLayer, PickAndPlaceExporter, PickAndPlaceOptions};
pub use draftsman::{
    DatumReference, DimensionKind, DraftsmanDocument, DraftsmanSheet, DrawingView, DrillTable,
    DrillTableRow, FeatureControlFrame, GeometricCharacteristic, HolePlating, MaterialCondition,
    SheetSize,
};
pub use drill::excellon::{DrillError, DrillHole, DrillHoleType, ExcellonExporter, ExcellonOutput};
pub use gerber::{GerberError, GerberExporter, GerberLayer, GerberLayerOutput, GerberOptions};
pub use outjob::{OutJobError, OutputJobConfig, OutputJobRunner, ReleasePackage};

pub use bom::{
    BomColumn, BomError, BomExporter, BomFormat, BomGrouping, BomIssueSeverity, BomMetadata,
    BomOptions, BomOutput, BomRow, BomRule, BomRuleOptions, BomTable, BomValidationIssue,
    BomValidationReport, rollup,
};
pub use netlist::{
    NetlistExporter, NetlistOptions, NetlistOutput, PSpiceNetlistError, PSpiceNetlistExporter,
    PSpiceNetlistOptions, PSpiceNetlistOutput,
};
pub use pdf::{
    ColourMode, Margins, Orientation, PageRange, PageSize, PdfExporter, PdfOptions, PdfOutput,
    PdfScale, SchematicPalette,
};
pub use preview::{PreviewOptions, PreviewPage, PreviewRasterizer};
pub use substitution::{SubstitutionContext, resolve};
pub use template::{Template, TemplateError, TemplateId, TitleBlockField};

/// The universal exporter trait — one impl per output format.
pub trait Exporter {
    type Options;
    type Output;
    type Error;

    fn export(
        &self,
        ctx: &ExportContext,
        opts: &Self::Options,
    ) -> Result<Self::Output, Self::Error>;
}

/// Everything an exporter needs to know about the project being exported.
///
/// The app layer builds this from `DocumentState` at export time — exporters
/// never touch the live application state directly.
#[derive(Debug, Clone)]
pub struct ExportContext {
    pub sheets: Vec<SheetSnapshot>,
    pub metadata: ProjectMetadata,
    /// The project's authoritative connectivity, derived once by the app via
    /// `oxide_net::build_project_netlist` and handed to exporters that emit
    /// connectivity (the netlist). `None` when the caller didn't derive it
    /// (e.g. a PDF-only export). Exporters read this rather than re-deriving
    /// nets from `sheets` (ADR-0002 D7).
    pub netlist: Option<oxide_types::net::Netlist>,
}

#[derive(Debug, Clone)]
pub struct SheetSnapshot {
    pub path: PathBuf,
    pub schematic: SchematicSheet,
    pub sheet_name: String,
    pub sheet_number: usize,
    pub sheet_count: usize,
}

/// Title-block / project-file metadata used to resolve `${TITLE}`, `${REV}`,
/// etc. and to stamp the exported artifact.
#[derive(Debug, Clone, Default)]
pub struct ProjectMetadata {
    pub title: String,
    pub revision: String,
    pub date: String,
    pub company: String,
    pub comments: [String; 4],
    pub custom_fields: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("bom: {0}")]
    Bom(#[from] bom::BomError),

    #[error("pdf: {0}")]
    Pdf(#[from] pdf::PdfError),

    #[error("netlist: {0}")]
    Netlist(#[from] netlist::NetlistError),

    #[error("template: {0}")]
    Template(#[from] template::TemplateError),
}
