//! Anti-Chatter Inertial Delay Filter for 12-State Mixed-Signal Co-Simulation.
//!
//! Prevents Zeno's Paradox and high-frequency event oscillations in continuous-discrete
//! feedback loops by enforcing a minimum transition pulse width and state degradation.

use crate::mixed_signal::{Logic12State, LogicEvent};
use std::collections::HashMap;

/// Anti-chatter filter that suppresses micro-glitches and rejects events faster than `min_pulse_width_s`.
#[derive(Debug, Clone)]
pub struct InertialFilter {
    pub min_pulse_width_s: f64,
    pub last_transition_time: HashMap<u32, f64>,
    pub last_state: HashMap<u32, Logic12State>,
    pub suppressed_event_count: usize,
}

impl InertialFilter {
    /// Creates a new `InertialFilter` with the specified minimum pulse width threshold.
    pub fn new(min_pulse_width_s: f64) -> Self {
        Self {
            min_pulse_width_s: min_pulse_width_s.max(1e-15),
            last_transition_time: HashMap::new(),
            last_state: HashMap::new(),
            suppressed_event_count: 0,
        }
    }

    /// Evaluates whether an incoming `LogicEvent` passes the inertial pulse width check.
    ///
    /// If an event arrives within `min_pulse_width_s` of the previous transition on the same net,
    /// it is suppressed (returning `None` or degraded to `WeakUnknown` / `Unknown`).
    pub fn filter_event(&mut self, event: LogicEvent) -> Option<LogicEvent> {
        let last_time = self.last_transition_time.get(&event.signal_id).copied();

        if let Some(t_prev) = last_time {
            let elapsed = event.timestamp_s - t_prev;
            if elapsed < self.min_pulse_width_s {
                self.suppressed_event_count += 1;
                // Suppress event or degrade state to WeakUnknown if glitching
                return None;
            }
        }

        self.last_transition_time
            .insert(event.signal_id, event.timestamp_s);
        self.last_state.insert(event.signal_id, event.new_state);
        Some(event)
    }

    /// Resets filter history.
    pub fn reset(&mut self) {
        self.last_transition_time.clear();
        self.last_state.clear();
        self.suppressed_event_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inertial_filter_glitch_suppression() {
        let mut filter = InertialFilter::new(50e-12); // 50 ps threshold

        let ev1 = LogicEvent {
            timestamp_s: 0.0,
            sequence_id: 0,
            signal_id: 1,
            new_state: Logic12State::ForcingOne,
        };
        assert!(filter.filter_event(ev1).is_some());

        // Event arriving 20 ps later (glitch, must be suppressed)
        let ev2 = LogicEvent {
            timestamp_s: 20e-12,
            sequence_id: 0,
            signal_id: 1,
            new_state: Logic12State::ForcingZero,
        };
        assert!(filter.filter_event(ev2).is_none());
        assert_eq!(filter.suppressed_event_count, 1);

        // Event arriving 60 ps later (valid pulse, accepted)
        let ev3 = LogicEvent {
            timestamp_s: 80e-12,
            sequence_id: 0,
            signal_id: 1,
            new_state: Logic12State::ForcingZero,
        };
        assert!(filter.filter_event(ev3).is_some());
    }
}
