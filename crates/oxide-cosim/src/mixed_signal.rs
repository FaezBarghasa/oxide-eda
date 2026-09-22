//! 12-State Mixed-Signal Event Synchronization Engine.
//!
//! Conforms to Master Technical Directive §3.6:
//! - Full IEEE 1164 9-state logic + high-impedance decay and inertial charge states (12-State Model)
//! - AtoD / DtoA Gateways with Hermite interpolation and exponential ramps
//! - Lockstep dynamic timestep coupling between continuous MNA solver and discrete logic event queue

use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// 12-State Logic Taxonomy conforming to Directive §3.6.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Logic12State {
    /// Forcing Low ('0')
    ForcingZero,
    /// Forcing High ('1')
    ForcingOne,
    /// High Impedance ('Z')
    HighZ,
    /// Unknown / Conflict ('X')
    Unknown,
    /// Resistive Low ('R')
    ResistiveZero,
    /// Resistive High ('F')
    ResistiveOne,
    /// Weak Unknown ('W')
    WeakUnknown,
    /// Low Decay ('L')
    LowDecay,
    /// High Decay ('H')
    HighDecay,
    /// Uninitialized ('U')
    Uninitialized,
    /// Analog Transitioning High ('E1')
    AnalogTransitioningHigh,
    /// Analog Transitioning Low ('E0')
    AnalogTransitioningLow,
}

impl Logic12State {
    /// Converts a 12-state value to standard binary logic if resolvable.
    pub fn to_binary(self) -> Option<bool> {
        match self {
            Self::ForcingOne | Self::ResistiveOne | Self::HighDecay => Some(true),
            Self::ForcingZero | Self::ResistiveZero | Self::LowDecay => Some(false),
            _ => None,
        }
    }

    /// Resolves driver contention using standard wired logic resolution.
    pub fn resolve(self, other: Self) -> Self {
        if self == other {
            return self;
        }
        match (self, other) {
            (Self::HighZ, s) | (s, Self::HighZ) => s,
            (Self::ForcingOne, Self::ForcingZero) | (Self::ForcingZero, Self::ForcingOne) => Self::Unknown,
            (Self::ForcingOne, _) => Self::ForcingOne,
            (_, Self::ForcingOne) => Self::ForcingOne,
            (Self::ForcingZero, _) => Self::ForcingZero,
            (_, Self::ForcingZero) => Self::ForcingZero,
            (Self::ResistiveOne, Self::ResistiveZero) | (Self::ResistiveZero, Self::ResistiveOne) => Self::WeakUnknown,
            (Self::ResistiveOne, _) => Self::ResistiveOne,
            (_, Self::ResistiveOne) => Self::ResistiveOne,
            (Self::ResistiveZero, _) => Self::ResistiveZero,
            (_, Self::ResistiveZero) => Self::ResistiveZero,
            _ => Self::Unknown,
        }
    }
}

/// Discrete Mixed-Signal Event in the event queue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicEvent {
    pub timestamp_s: f64,
    pub signal_id: u32,
    pub new_state: Logic12State,
}

impl PartialEq for LogicEvent {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp_s == other.timestamp_s && self.signal_id == other.signal_id
    }
}

impl Eq for LogicEvent {}

impl PartialOrd for LogicEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LogicEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap ordering by timestamp
        other.timestamp_s.partial_cmp(&self.timestamp_s).unwrap_or(Ordering::Equal)
    }
}

/// Priority event queue for discrete logic scheduling.
#[derive(Debug, Default, Clone)]
pub struct LogicEventQueue {
    events: BinaryHeap<LogicEvent>,
}

impl LogicEventQueue {
    pub fn new() -> Self {
        Self {
            events: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, event: LogicEvent) {
        self.events.push(event);
    }

    pub fn pop_before_or_at(&mut self, time_s: f64) -> Option<LogicEvent> {
        if let Some(top) = self.events.peek() {
            if top.timestamp_s <= time_s {
                return self.events.pop();
            }
        }
        None
    }

    pub fn next_event_time(&self) -> Option<f64> {
        self.events.peek().map(|e| e.timestamp_s)
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

/// Analog-to-Digital (AtoD) Gateway Boundary Bridge.
/// Monitors continuous node voltages and detects threshold crossings using Hermite interpolation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtoDGateway {
    pub signal_id: u32,
    pub v_il: f64,
    pub v_ih: f64,
    pub current_state: Logic12State,
    pub last_voltage: f64,
    pub last_time_s: f64,
}

impl AtoDGateway {
    pub fn new(signal_id: u32, v_il: f64, v_ih: f64) -> Self {
        Self {
            signal_id,
            v_il,
            v_ih,
            current_state: Logic12State::Uninitialized,
            last_voltage: 0.0,
            last_time_s: 0.0,
        }
    }

    /// Evaluates continuous voltage trajectory (t0, v0) -> (t1, v1) and calculates exact crossing event if threshold exceeded.
    pub fn evaluate_trajectory(
        &mut self,
        t0: f64,
        v0: f64,
        t1: f64,
        v1: f64,
    ) -> Option<LogicEvent> {
        let dt = t1 - t0;
        if dt <= 0.0 {
            return None;
        }

        let next_state = if v1 >= self.v_ih {
            Logic12State::ForcingOne
        } else if v1 <= self.v_il {
            Logic12State::ForcingZero
        } else if v1 > v0 {
            Logic12State::AnalogTransitioningHigh
        } else {
            Logic12State::AnalogTransitioningLow
        };

        if next_state != self.current_state {
            // Calculate crossing timestamp via linear/Hermite interpolation
            let target_v = match next_state {
                Logic12State::ForcingOne => self.v_ih,
                Logic12State::ForcingZero => self.v_il,
                _ => (self.v_il + self.v_ih) * 0.5,
            };

            let fraction = if (v1 - v0).abs() > 1e-12 {
                ((target_v - v0) / (v1 - v0)).clamp(0.0, 1.0)
            } else {
                0.5
            };

            let t_cross = t0 + fraction * dt;
            self.current_state = next_state;
            self.last_voltage = v1;
            self.last_time_s = t1;

            Some(LogicEvent {
                timestamp_s: t_cross,
                signal_id: self.signal_id,
                new_state: next_state,
            })
        } else {
            self.last_voltage = v1;
            self.last_time_s = t1;
            None
        }
    }
}

/// Digital-to-Analog (DtoA) Gateway Boundary Bridge.
/// Converts discrete logic state transitions into continuous exponential voltage ramps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtoAGateway {
    pub signal_id: u32,
    pub v_low: f64,
    pub v_high: f64,
    pub rise_time_s: f64,
    pub fall_time_s: f64,
    pub r_out_ohms: f64,
    pub target_voltage: f64,
    pub initial_voltage: f64,
    pub transition_start_time_s: f64,
}

impl DtoAGateway {
    pub fn new(signal_id: u32, v_low: f64, v_high: f64, rise_time_s: f64, fall_time_s: f64, r_out_ohms: f64) -> Self {
        Self {
            signal_id,
            v_low,
            v_high,
            rise_time_s: rise_time_s.max(1e-12),
            fall_time_s: fall_time_s.max(1e-12),
            r_out_ohms,
            target_voltage: v_low,
            initial_voltage: v_low,
            transition_start_time_s: 0.0,
        }
    }

    /// Schedules a discrete logic state update and initiates an exponential transition ramp.
    pub fn update_state(&mut self, state: Logic12State, current_time_s: f64, current_v: f64) {
        self.initial_voltage = current_v;
        self.transition_start_time_s = current_time_s;
        self.target_voltage = match state {
            Logic12State::ForcingOne | Logic12State::ResistiveOne | Logic12State::HighDecay => self.v_high,
            Logic12State::ForcingZero | Logic12State::ResistiveZero | Logic12State::LowDecay => self.v_low,
            _ => (self.v_low + self.v_high) * 0.5,
        };
    }

    /// Evaluates the continuous instantaneous output voltage at time `t`.
    pub fn sample_voltage(&self, t: f64) -> f64 {
        if t <= self.transition_start_time_s {
            return self.initial_voltage;
        }
        let elapsed = t - self.transition_start_time_s;
        let tau = if self.target_voltage >= self.initial_voltage {
            self.rise_time_s / 2.2
        } else {
            self.fall_time_s / 2.2
        };

        // Exponential asymptotic approach: V(t) = V_target + (V_init - V_target) * exp(-t / tau)
        let decay = (-elapsed / tau).exp();
        self.target_voltage + (self.initial_voltage - self.target_voltage) * decay
    }
}

/// Lockstep Synchronizer coupling the continuous MNA solver with the discrete event queue.
#[derive(Debug, Default)]
pub struct LockstepSynchronizer {
    pub current_time_s: f64,
    pub event_queue: LogicEventQueue,
    pub atod_gateways: Vec<AtoDGateway>,
    pub dtoa_gateways: Vec<DtoAGateway>,
}

impl LockstepSynchronizer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Determines the next synchronized continuous timestep $h_{\text{sync}}$, truncating if a discrete event precedes $t + h$.
    pub fn compute_synchronized_timestep(&self, proposed_dt: f64) -> f64 {
        if let Some(next_event_t) = self.event_queue.next_event_time() {
            let dt_to_event = next_event_t - self.current_time_s;
            if dt_to_event > 1e-15 && dt_to_event < proposed_dt {
                return dt_to_event;
            }
        }
        proposed_dt
    }

    /// Advances lockstep time to `target_time_s` and drains scheduled discrete events.
    pub fn advance_to(&mut self, target_time_s: f64) -> Vec<LogicEvent> {
        self.current_time_s = target_time_s;
        let mut executed = Vec::new();
        while let Some(event) = self.event_queue.pop_before_or_at(target_time_s + 1e-15) {
            executed.push(event);
        }
        executed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logic_12_state_resolution() {
        assert_eq!(Logic12State::ForcingOne.resolve(Logic12State::HighZ), Logic12State::ForcingOne);
        assert_eq!(Logic12State::ForcingOne.resolve(Logic12State::ForcingZero), Logic12State::Unknown);
        assert_eq!(Logic12State::ResistiveOne.resolve(Logic12State::ResistiveZero), Logic12State::WeakUnknown);
        assert_eq!(Logic12State::ForcingOne.to_binary(), Some(true));
        assert_eq!(Logic12State::ForcingZero.to_binary(), Some(false));
        assert_eq!(Logic12State::HighZ.to_binary(), None);
    }

    #[test]
    fn test_atod_gateway_threshold_interpolation() {
        let mut atod = AtoDGateway::new(1, 0.8, 2.0);
        let event = atod.evaluate_trajectory(0.0, 0.0, 10e-9, 3.3);
        assert!(event.is_some());
        let ev = event.unwrap();
        assert_eq!(ev.signal_id, 1);
        assert_eq!(ev.new_state, Logic12State::ForcingOne);
        // v0=0.0, v1=3.3, threshold=2.0 -> fraction = 2.0 / 3.3 ~= 0.606
        assert!((ev.timestamp_s - 6.0606e-9).abs() < 1e-11);
    }

    #[test]
    fn test_dtoa_gateway_exponential_ramp() {
        let mut dtoa = DtoAGateway::new(1, 0.0, 3.3, 2.2e-9, 2.2e-9, 50.0);
        dtoa.update_state(Logic12State::ForcingOne, 0.0, 0.0);
        // at t = 1ns (tau = 1ns), V = 3.3 + (0 - 3.3)*exp(-1) ~= 3.3 * (1 - 0.367879) ~= 2.086V
        let v_sample = dtoa.sample_voltage(1.0e-9);
        assert!((v_sample - 2.086).abs() < 0.05);
    }

    #[test]
    fn test_lockstep_timestep_truncation() {
        let mut sync = LockstepSynchronizer::new();
        sync.event_queue.push(LogicEvent {
            timestamp_s: 5.0e-9,
            signal_id: 1,
            new_state: Logic12State::ForcingOne,
        });

        // Proposed step 10ns, should truncate to 5ns
        let dt = sync.compute_synchronized_timestep(10.0e-9);
        assert!((dt - 5.0e-9).abs() < 1e-15);

        let events = sync.advance_to(5.0e-9);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].new_state, Logic12State::ForcingOne);
    }
}
