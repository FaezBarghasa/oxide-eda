//! Basic, General-Purpose, and Advanced Timers with PWM & Dead-time Generation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TimerCounterMode {
    #[default]
    Up,
    Down,
    CenterAligned1,
    CenterAligned2,
    CenterAligned3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TimerChannelMode {
    #[default]
    Disabled,
    PwmMode1,
    PwmMode2,
    InputCapture,
    OutputCompare,
}

/// A single Timer Capture/Compare Channel.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct TimerChannel {
    pub mode: TimerChannelMode,
    pub ccr: u32,
    pub duty_cycle: f64,
    pub output_state: bool,
    pub complementary_output: bool,
}

/// Hardware Timer & PWM peripheral model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerPeripheral {
    pub name: String,
    pub enabled: bool,
    pub counter_mode: TimerCounterMode,
    pub prescaler: u16,
    pub auto_reload: u32,
    pub counter: u32,
    pub repetition_counter: u8,
    pub dead_time_cycles: u8,
    pub channels: [TimerChannel; 4],
}

impl TimerPeripheral {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            enabled: false,
            counter_mode: TimerCounterMode::Up,
            prescaler: 0,
            auto_reload: 1000,
            counter: 0,
            repetition_counter: 0,
            dead_time_cycles: 0,
            channels: [TimerChannel::default(); 4],
        }
    }

    pub fn set_pwm_channel(&mut self, channel_idx: usize, ccr_val: u32) {
        if channel_idx < 4 {
            self.channels[channel_idx].mode = TimerChannelMode::PwmMode1;
            self.channels[channel_idx].ccr = ccr_val;
            self.update_channel_duty(channel_idx);
        }
    }

    fn update_channel_duty(&mut self, channel_idx: usize) {
        if self.auto_reload > 0 {
            let ccr = self.channels[channel_idx].ccr;
            let duty = (ccr as f64 / self.auto_reload as f64).clamp(0.0, 1.0);
            self.channels[channel_idx].duty_cycle = duty;
            self.channels[channel_idx].output_state = duty > 0.0;
        }
    }

    /// Advances timer counter by clock cycles.
    /// Returns `true` if an update event (overflow/underflow interrupt) occurred.
    pub fn step_cycles(&mut self, cycles: u32) -> bool {
        if !self.enabled || self.auto_reload == 0 {
            return false;
        }

        let prescaled_cycles = cycles / (self.prescaler as u32 + 1);
        if prescaled_cycles == 0 {
            return false;
        }

        let mut update_event = false;
        match self.counter_mode {
            TimerCounterMode::Up => {
                self.counter += prescaled_cycles;
                if self.counter >= self.auto_reload {
                    self.counter %= self.auto_reload + 1;
                    update_event = true;
                }
            }
            TimerCounterMode::Down => {
                if self.counter <= prescaled_cycles {
                    let rem = prescaled_cycles - self.counter;
                    self.counter = self.auto_reload.saturating_sub(rem % (self.auto_reload + 1));
                    update_event = true;
                } else {
                    self.counter -= prescaled_cycles;
                }
            }
            _ => {
                // Center-aligned triangle counter
                self.counter = (self.counter + prescaled_cycles) % (self.auto_reload * 2);
                if self.counter >= self.auto_reload {
                    update_event = true;
                }
            }
        }

        // Update PWM channel instantaneous output states
        for ch in &mut self.channels {
            if ch.mode == TimerChannelMode::PwmMode1 {
                ch.output_state = self.counter < ch.ccr;
            } else if ch.mode == TimerChannelMode::PwmMode2 {
                ch.output_state = self.counter >= ch.ccr;
            }
        }

        update_event
    }
}
