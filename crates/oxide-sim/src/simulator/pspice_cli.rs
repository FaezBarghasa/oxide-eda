//! External Cadence OrCAD PSpice / PSpice for TI CLI simulator adapter.

use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::future::Future;
use oxide_types::sim::WaveformDataset;

use crate::parser::parse_csdf;
use crate::simulator::{SimError, SimProgress, Simulator};

/// External Cadence OrCAD PSpice batch runner adapter.
#[derive(Debug, Clone)]
pub struct PSpiceCliSimulator {
    binary_path: PathBuf,
}

impl Default for PSpiceCliSimulator {
    fn default() -> Self {
        Self::new("pspice")
    }
}

impl PSpiceCliSimulator {
    pub fn new(binary: impl Into<PathBuf>) -> Self {
        Self {
            binary_path: binary.into(),
        }
    }
}

impl Simulator for PSpiceCliSimulator {
    fn id(&self) -> &'static str {
        "cadence_pspice_cli"
    }

    fn display_name(&self) -> &'static str {
        "Cadence OrCAD PSpice CLI"
    }

    fn run(
        &self,
        deck: &str,
        work_dir: &Path,
        progress_tx: Option<tokio::sync::mpsc::Sender<SimProgress>>,
    ) -> Pin<Box<dyn Future<Output = Result<WaveformDataset, SimError>> + Send + '_>> {
        let deck_string = deck.to_string();
        let dir = work_dir.to_path_buf();
        let bin = self.binary_path.clone();

        Box::pin(async move {
            let cir_path = dir.join("circuit.cir");
            let out_path = dir.join("circuit.out");
            let csdf_path = dir.join("circuit.csd");

            if let Some(tx) = &progress_tx {
                let _ = tx.send(SimProgress {
                    percent: Some(0.1),
                    message: "Writing PSpice circuit deck...".to_string(),
                }).await;
            }

            tokio::fs::write(&cir_path, deck_string.as_bytes()).await?;

            if let Some(tx) = &progress_tx {
                let _ = tx.send(SimProgress {
                    percent: Some(0.3),
                    message: "Invoking Cadence PSpice batch solver...".to_string(),
                }).await;
            }

            let output = tokio::process::Command::new(&bin)
                .arg("-b")
                .arg(&cir_path)
                .current_dir(&dir)
                .output()
                .await
                .map_err(|e| {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        SimError::BinaryNotFound {
                            name: bin.display().to_string(),
                            details: "Cadence PSpice executable not found in PATH or specified location".to_string(),
                        }
                    } else {
                        SimError::Io(e)
                    }
                })?;

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let mut log_lines: Vec<String> = Vec::new();
            for l in stdout.lines().chain(stderr.lines()) {
                log_lines.push(l.to_string());
            }

            if csdf_path.exists() {
                let text = tokio::fs::read_to_string(&csdf_path).await?;
                let mut dataset = parse_csdf(&text)?;
                dataset.log = log_lines;
                return Ok(dataset);
            }

            if out_path.exists() {
                let text = tokio::fs::read_to_string(&out_path).await?;
                let mut dataset = parse_csdf(&text).unwrap_or_else(|_| WaveformDataset::empty("PSpice Output"));
                dataset.log = log_lines;
                return Ok(dataset);
            }

            Err(SimError::ExecutionFailed {
                exit_code: output.status.code(),
                message: if !stderr.is_empty() { stderr.to_string() } else { stdout.to_string() },
            })
        })
    }
}
