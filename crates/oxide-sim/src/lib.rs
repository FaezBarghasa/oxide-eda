//! `oxide-sim` — Circuit simulation engine and PSpice deck generator for Oxide EDA.
//!
//! Provides the [`Simulator`] port trait with concrete adapters:
//! - [`NgSpiceSimulator`]: Local/built-in ngspice running in PSpice mode (`set ngbehavior=ps`).
//! - [`PSpiceCliSimulator`]: External Cadence OrCAD PSpice CLI batch runner.
//!
//! Also provides the [`PSpiceDeckBuilder`] for translating schematic connectivity
//! and component models into valid PSpice simulation decks, plus result parsers
//! and mathematical analysis helpers.

pub mod analysis;
pub mod deck;
pub mod engine;
pub mod parser;
pub mod simulator;

pub use analysis::{TraceStats, calculate_rise_time, calculate_stats};
pub use deck::PSpiceDeckBuilder;
pub use engine::{ConvergenceStage, InProcessMnaSolver, MnaSolver, StepTelemetry};
pub use parser::{parse_csdf, parse_spice_raw};
pub use simulator::{NgSpiceSimulator, PSpiceCliSimulator, SimError, SimProgress, Simulator};
