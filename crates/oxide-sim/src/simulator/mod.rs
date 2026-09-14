//! Simulator trait (port) and common error/result types.

use std::path::Path;
use thiserror::Error;
use oxide_types::sim::WaveformDataset;

pub mod ngspice;
pub mod pspice_cli;

pub use ngspice::NgSpiceSimulator;
pub use pspice_cli::PSpiceCliSimulator;

/// Error types occurring during circuit simulation.
#[derive(Debug, Error)]
pub enum SimError {
    #[error("simulator binary '{name}' not found: {details}")]
    BinaryNotFound { name: String, details: String },

    #[error("simulation execution failed (exit code {exit_code:?}): {message}")]
    ExecutionFailed {
        exit_code: Option<i32>,
        message: String,
    },

    #[error("I/O error during simulation: {0}")]
    Io(#[from] std::io::Error),

    #[error("error parsing simulation results: {0}")]
    ParseError(String),

    #[error("simulation cancelled by user")]
    Cancelled,

    #[error("simulation convergence error: {0}")]
    ConvergenceError(String),

    #[error("invalid circuit deck: {0}")]
    InvalidDeck(String),
}

/// Simulation progress report.
#[derive(Debug, Clone)]
pub struct SimProgress {
    pub percent: Option<f32>,
    pub message: String,
}

/// The abstraction port for external circuit simulators (ADR-0001 §A3.2).
pub trait Simulator: Send + Sync {
    /// Identifier of this simulator backend.
    fn id(&self) -> &'static str;

    /// User-visible display name.
    fn display_name(&self) -> &'static str;

    /// Run the simulation deck asynchronously and parse results into a [`WaveformDataset`].
    fn run(
        &self,
        deck: &str,
        work_dir: &Path,
        progress_tx: Option<tokio::sync::mpsc::Sender<SimProgress>>,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<WaveformDataset, SimError>> + Send + '_>,
    >;
}
