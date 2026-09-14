//! Watchdog Timers: Independent Watchdog (IWDG) & Window Watchdog (WWDG).

use serde::{Deserialize, Serialize};

/// Independent Watchdog (IWDG) clocked by dedicated low-speed oscillator (LSI ~32 kHz).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Iwdg {
    pub enabled: bool,
    pub prescaler: u16,
    pub reload_value: u16,
    pub counter: u16,
    pub reset_tripped: bool,
}

impl Default for Iwdg {
    fn default() -> Self {
        Self::new()
    }
}

impl Iwdg {
    pub fn new() -> Self {
        Self {
            enabled: false,
            prescaler: 4,
            reload_value: 0x0FFF,
            counter: 0x0FFF,
            reset_tripped: false,
        }
    }

    /// Key write emulation: `0xCCCC` starts watchdog, `0xAAAA` reloads counter.
    pub fn write_key(&mut self, key: u16) {
        match key {
            0xCCCC => {
                self.enabled = true;
                self.counter = self.reload_value;
            }
            0xAAAA => {
                self.counter = self.reload_value;
            }
            _ => {}
        }
    }

    /// Advance watchdog by LSI clock ticks.
    pub fn step_ticks(&mut self, ticks: u16) -> bool {
        if !self.enabled {
            return false;
        }

        if self.counter <= ticks {
            self.counter = 0;
            self.reset_tripped = true;
            true
        } else {
            self.counter -= ticks;
            false
        }
    }
}

/// Window Watchdog (WWDG).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wwdg {
    pub enabled: bool,
    pub window: u8,
    pub counter: u8,
    pub reset_tripped: bool,
}

impl Default for Wwdg {
    fn default() -> Self {
        Self::new()
    }
}

impl Wwdg {
    pub fn new() -> Self {
        Self {
            enabled: false,
            window: 0x7F,
            counter: 0x7F,
            reset_tripped: false,
        }
    }
}

/// Aggregated Watchdog Subsystem.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WatchdogPeripheral {
    pub iwdg: Iwdg,
    pub wwdg: Wwdg,
}
