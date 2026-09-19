//! Interactive routing session state.

use oxide_physics::Microns;

use super::RoutingMode;
use crate::geometry::Point2D;
use crate::geometry::rtree::NetId;
use crate::{LayerId, RouteSegment, ViaPlacement};

/// Corner routing styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CornerStyle {
    #[default]
    Mitred45,
    Arc45,
    RightAngle90,
    FullArc,
}

impl CornerStyle {
    pub fn next(&self) -> Self {
        match self {
            Self::Mitred45 => Self::Arc45,
            Self::Arc45 => Self::RightAngle90,
            Self::RightAngle90 => Self::FullArc,
            Self::FullArc => Self::Mitred45,
        }
    }
}

/// Live interactive HUD state for high-speed length and phase tuning.
#[derive(Debug, Clone, PartialEq)]
pub struct InteractiveTuningHudState {
    pub target_length_microns: Microns,
    pub current_length_microns: Microns,
    pub package_delay_microns: Microns,
    pub tolerance_microns: Microns,
    pub amplitude_microns: Microns,
    pub pitch_microns: Microns,
    pub corner_style: CornerStyle,
}

impl InteractiveTuningHudState {
    pub fn new(target_length_microns: Microns) -> Self {
        Self {
            target_length_microns,
            current_length_microns: 0,
            package_delay_microns: 0,
            tolerance_microns: 50, // 50 µm (~2 mil)
            amplitude_microns: 800, // 800 µm amplitude
            pitch_microns: 600,     // 600 µm pitch
            corner_style: CornerStyle::Mitred45,
        }
    }

    /// Effective total length including trace length + package internal bond delay.
    pub fn total_effective_length(&self) -> Microns {
        self.current_length_microns + self.package_delay_microns
    }

    /// Deviation from target length in micrometers (+ is over, - is under).
    pub fn length_delta(&self) -> Microns {
        self.total_effective_length() - self.target_length_microns
    }

    /// Whether current length meets target within defined tolerance.
    pub fn is_in_tolerance(&self) -> bool {
        self.length_delta().abs() <= self.tolerance_microns
    }

    /// Adjust amplitude with hotkeys (e.g. '1' = +100µm, '2' = -100µm).
    pub fn adjust_amplitude(&mut self, delta_microns: Microns) {
        self.amplitude_microns = (self.amplitude_microns + delta_microns).max(100);
    }

    /// Adjust pitch/wavelength with hotkeys (e.g. '3' = +100µm, '4' = -100µm).
    pub fn adjust_pitch(&mut self, delta_microns: Microns) {
        self.pitch_microns = (self.pitch_microns + delta_microns).max(100);
    }
}

/// Active state while user is dragging or clicking to lay down a trace.
#[derive(Debug, Clone)]
pub struct RoutingSession {
    pub current_net: NetId,
    pub current_position: Point2D,
    pub current_layer: LayerId,
    pub segments_placed: Vec<RouteSegment>,
    pub vias_placed: Vec<ViaPlacement>,
    pub mode: RoutingMode,
    pub track_width: Microns,
    pub tuning_hud: Option<InteractiveTuningHudState>,
    pub corner_style: CornerStyle,
}

impl RoutingSession {
    pub fn commit_segment(&mut self, segment: RouteSegment) {
        self.current_position = segment.end_point;
        self.segments_placed.push(segment);
    }

    pub fn commit_via(&mut self, via: ViaPlacement) {
        self.current_layer = via.end_layer;
        self.current_position = via.position;
        self.vias_placed.push(via);
    }
}
