//! NgSpice simulator adapter operating in PSpice compatibility mode (`set ngbehavior=ps`).

use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::future::Future;
use oxide_types::sim::WaveformDataset;

use crate::parser::parse_spice_raw;
use crate::simulator::{SimError, SimProgress, Simulator};

/// Local ngspice runner executing in PSpice compatibility mode.
#[derive(Debug, Clone)]
pub struct NgSpiceSimulator {
    binary_path: PathBuf,
}

impl Default for NgSpiceSimulator {
    fn default() -> Self {
        Self::new("ngspice")
    }
}

impl NgSpiceSimulator {
    pub fn new(binary: impl Into<PathBuf>) -> Self {
        Self {
            binary_path: binary.into(),
        }
    }

    /// Check if the ngspice binary is executable.
    pub async fn is_available(&self) -> bool {
        tokio::process::Command::new(&self.binary_path)
            .arg("-v")
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

impl Simulator for NgSpiceSimulator {
    fn id(&self) -> &'static str {
        "ngspice_pspice"
    }

    fn display_name(&self) -> &'static str {
        "Built-in SPICE (ngspice in PSpice mode)"
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
            let cir_path = dir.join("simulation.cir");
            let raw_path = dir.join("simulation.raw");

            if let Some(tx) = &progress_tx {
                let _ = tx.send(SimProgress {
                    percent: Some(0.1),
                    message: "Writing PSpice circuit deck...".to_string(),
                }).await;
            }

            // Inject PSpice behavior directive at start of deck
            let mut final_deck = String::new();
            final_deck.push_str("* Oxide EDA - PSpice Simulation Run\n");
            final_deck.push_str(".control\nset ngbehavior=ps\n.endc\n");
            final_deck.push_str(&deck_string);

            // Write circuit deck to disk
            tokio::fs::write(&cir_path, final_deck.as_bytes()).await?;

            // Remove any old .raw file
            let _ = tokio::fs::remove_file(&raw_path).await;

            if let Some(tx) = &progress_tx {
                let _ = tx.send(SimProgress {
                    percent: Some(0.3),
                    message: "Starting simulation engine...".to_string(),
                }).await;
            }

            // Execute ngspice in batch mode
            let output = tokio::process::Command::new(&bin)
                .arg("-b")
                .arg("-r")
                .arg(&raw_path)
                .arg(&cir_path)
                .current_dir(&dir)
                .output()
                .await
                .map_err(|e| {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        SimError::BinaryNotFound {
                            name: bin.display().to_string(),
                            details: "Install ngspice (`sudo apt install ngspice` on Ubuntu/Pop!_OS)".to_string(),
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

            if !output.status.success() {
                // Check if raw file was produced anyway (sometimes ngspice returns non-zero on warnings)
                if !raw_path.exists() {
                    return Err(SimError::ExecutionFailed {
                        exit_code: output.status.code(),
                        message: if !stderr.is_empty() { stderr.to_string() } else { stdout.to_string() },
                    });
                }
            }

            if let Some(tx) = &progress_tx {
                let _ = tx.send(SimProgress {
                    percent: Some(0.8),
                    message: "Parsing simulation waveforms...".to_string(),
                }).await;
            }

            if !raw_path.exists() {
                return Err(SimError::ExecutionFailed {
                    exit_code: output.status.code(),
                    message: format!("No raw simulation results generated.\n{}", stdout),
                });
            }

            let raw_bytes = tokio::fs::read(&raw_path).await?;
            let mut dataset = parse_spice_raw(&raw_bytes)?;
            dataset.log = log_lines;

            if let Some(tx) = &progress_tx {
                let _ = tx.send(SimProgress {
                    percent: Some(1.0),
                    message: "Simulation completed successfully.".to_string(),
                }).await;
            }

            Ok(dataset)
        })
    }
}
