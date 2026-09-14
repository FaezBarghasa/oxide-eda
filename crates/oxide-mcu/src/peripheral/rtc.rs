//! Real-Time Clock (RTC) & Calendar Emulation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RtcCalendarTime {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub subseconds: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RtcAlarm {
    pub enabled: bool,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

/// Real-Time Clock Hardware peripheral model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtcPeripheral {
    pub name: String,
    pub time: RtcCalendarTime,
    pub alarm_a: RtcAlarm,
    pub alarm_b: RtcAlarm,
    pub alarm_tripped: bool,
}

impl Default for RtcPeripheral {
    fn default() -> Self {
        Self::new("RTC")
    }
}

impl RtcPeripheral {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            time: RtcCalendarTime {
                hours: 12,
                minutes: 0,
                seconds: 0,
                year: 2026,
                month: 9,
                day: 14,
                subseconds: 0,
            },
            alarm_a: RtcAlarm::default(),
            alarm_b: RtcAlarm::default(),
            alarm_tripped: false,
        }
    }

    /// Advances RTC clock by 1 second.
    pub fn tick_second(&mut self) {
        self.time.seconds += 1;
        if self.time.seconds >= 60 {
            self.time.seconds = 0;
            self.time.minutes += 1;
            if self.time.minutes >= 60 {
                self.time.minutes = 0;
                self.time.hours = (self.time.hours + 1) % 24;
            }
        }

        // Check alarms
        if self.alarm_a.enabled && self.alarm_a.hours == self.time.hours && self.alarm_a.minutes == self.time.minutes && self.alarm_a.seconds == self.time.seconds {
            self.alarm_tripped = true;
        }
    }
}
