//! ARM Core 24-bit SysTick Timer Emulation.

use serde::{Deserialize, Serialize};

/// ARM SysTick 24-bit core system timer.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SysTick {
    pub enabled: bool,
    pub tick_interrupt: bool,
    pub clk_source_cpu: bool,
    pub count_flag: bool,
    pub reload_value: u32,
    pub current_value: u32,
}

impl SysTick {
    pub fn new() -> Self {
        Self {
            enabled: false,
            tick_interrupt: false,
            clk_source_cpu: true,
            count_flag: false,
            reload_value: 0x00FF_FFFF,
            current_value: 0x00FF_FFFF,
        }
    }

    /// Advances the timer by N core clock cycles.
    /// Returns `true` if a SysTick exception / interrupt should be triggered.
    pub fn step_cycles(&mut self, cycles: u32) -> bool {
        if !self.enabled || self.reload_value == 0 {
            return false;
        }

        let mut fired = false;
        if self.current_value <= cycles {
            let rem = cycles - self.current_value;
            self.count_flag = true;
            if self.tick_interrupt {
                fired = true;
            }
            self.current_value = self.reload_value.saturating_sub(rem % (self.reload_value + 1));
        } else {
            self.current_value -= cycles;
        }

        fired
    }

    pub fn set_reload(&mut self, reload: u32) {
        self.reload_value = reload & 0x00FF_FFFF;
    }

    pub fn reset_counter(&mut self) {
        self.current_value = self.reload_value;
        self.count_flag = false;
    }
}
