//! Simulation types and data structures for Oxide EDA.

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

/// Type of AC frequency sweep.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AcSweepType {
    #[default]
    Decade,
    Octave,
    Linear,
}

/// Nested secondary DC sweep source configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcNestedSweep {
    pub source: String,
    pub start: f64,
    pub stop: f64,
    pub step: f64,
}

/// Simulation analysis directive and parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AnalysisKind {
    Transient {
        #[serde(default)]
        start_time: f64,
        stop_time: f64,
        step_time: f64,
        #[serde(default)]
        max_step: Option<f64>,
        #[serde(default)]
        uic: bool,
    },
    Ac {
        #[serde(default)]
        sweep_type: AcSweepType,
        points: usize,
        start_freq: f64,
        stop_freq: f64,
    },
    Dc {
        source: String,
        start: f64,
        stop: f64,
        step: f64,
        #[serde(default)]
        nested: Option<DcNestedSweep>,
    },
    OperatingPoint,
    Parametric {
        param_name: String,
        sweep_values: Vec<f64>,
    },
    Temperature {
        temps: Vec<f64>,
    },
}

impl Default for AnalysisKind {
    fn default() -> Self {
        Self::Transient {
            start_time: 0.0,
            stop_time: 1e-3, // 1 ms
            step_time: 1e-6, // 1 µs
            max_step: None,
            uic: false,
        }
    }
}

/// Simulation probe target.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Probe {
    Voltage {
        net_name: String,
        #[serde(default)]
        net_id: Option<u32>,
    },
    Current {
        component_ref: String,
        pin: String,
    },
    Differential {
        pos_net: String,
        neg_net: String,
    },
}

impl Probe {
    pub fn display_name(&self) -> String {
        match self {
            Probe::Voltage { net_name, .. } => format!("V({})", net_name),
            Probe::Current { component_ref, pin } => format!("I({}:{})", component_ref, pin),
            Probe::Differential { pos_net, neg_net } => format!("V({}, {})", pos_net, neg_net),
        }
    }
}

/// Simulation execution settings and options.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub analysis: AnalysisKind,
    #[serde(default)]
    pub probes: Vec<Probe>,
    #[serde(default = "default_temperature")]
    pub temp_c: f64,
    #[serde(default = "default_reltol")]
    pub reltol: f64,
    #[serde(default = "default_vntol")]
    pub vntol: f64,
    #[serde(default = "default_abstol")]
    pub abstol: f64,
    #[serde(default)]
    pub custom_options: BTreeMap<String, String>,
}

fn default_temperature() -> f64 {
    27.0
}
fn default_reltol() -> f64 {
    0.001
}
fn default_vntol() -> f64 {
    1e-6
}
fn default_abstol() -> f64 {
    1e-12
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            analysis: AnalysisKind::default(),
            probes: Vec::new(),
            temp_c: default_temperature(),
            reltol: default_reltol(),
            vntol: default_vntol(),
            abstol: default_abstol(),
            custom_options: BTreeMap::new(),
        }
    }
}

/// Physical unit for a waveform trace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraceUnit {
    TimeSeconds,
    FrequencyHertz,
    VoltageVolts,
    CurrentAmperes,
    PowerWatts,
    PhaseDegrees,
    MagnitudeDecibels,
    Dimensionless,
}

impl TraceUnit {
    pub fn suffix(&self) -> &'static str {
        match self {
            TraceUnit::TimeSeconds => "s",
            TraceUnit::FrequencyHertz => "Hz",
            TraceUnit::VoltageVolts => "V",
            TraceUnit::CurrentAmperes => "A",
            TraceUnit::PowerWatts => "W",
            TraceUnit::PhaseDegrees => "°",
            TraceUnit::MagnitudeDecibels => "dB",
            TraceUnit::Dimensionless => "",
        }
    }
}

/// A single waveform channel/trace vector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaveformTrace {
    pub name: String,
    pub unit: TraceUnit,
    pub values: Vec<f64>,
}

/// Complete dataset returned by a simulation run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaveformDataset {
    pub title: String,
    pub analysis_name: String,
    /// The X-axis independent variable (e.g. time, frequency, or sweep voltage).
    pub x_trace: WaveformTrace,
    /// The Y-axis dependent signals (voltages, currents, expressions).
    pub traces: Vec<WaveformTrace>,
    /// Optional DC operating point result table (node voltages, branch currents).
    #[serde(default)]
    pub operating_point: BTreeMap<String, f64>,
    /// Console output / log generated during the simulation.
    #[serde(default)]
    pub log: Vec<String>,
}

impl WaveformDataset {
    pub fn empty(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            analysis_name: "Transient".to_string(),
            x_trace: WaveformTrace {
                name: "time".to_string(),
                unit: TraceUnit::TimeSeconds,
                values: Vec::new(),
            },
            traces: Vec::new(),
            operating_point: BTreeMap::new(),
            log: Vec::new(),
        }
    }

    pub fn get_trace(&self, name: &str) -> Option<&WaveformTrace> {
        self.traces.iter().find(|t| t.name.eq_ignore_ascii_case(name))
    }
}
