//! Nested Vectored Interrupt Controller (NVIC) Emulation.

use serde::{Deserialize, Serialize};

pub const MAX_INTERRUPTS: usize = 256;

/// State for a single interrupt line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct InterruptLine {
    pub enabled: bool,
    pub pending: bool,
    pub active: bool,
    pub priority: u8,
}

/// ARM Nested Vectored Interrupt Controller (NVIC).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nvic {
    pub lines: Vec<InterruptLine>,
    pub vtor: u32,
    pub primask: bool,
    pub basepri: u8,
}

impl Default for Nvic {
    fn default() -> Self {
        Self::new()
    }
}

impl Nvic {
    pub fn new() -> Self {
        Self {
            lines: vec![InterruptLine::default(); MAX_INTERRUPTS],
            vtor: 0x0800_0000,
            primask: false,
            basepri: 0,
        }
    }

    pub fn enable_irq(&mut self, irq: usize) {
        if irq < self.lines.len() {
            self.lines[irq].enabled = true;
        }
    }

    pub fn disable_irq(&mut self, irq: usize) {
        if irq < self.lines.len() {
            self.lines[irq].enabled = false;
        }
    }

    pub fn set_pending(&mut self, irq: usize) {
        if irq < self.lines.len() {
            self.lines[irq].pending = true;
        }
    }

    pub fn clear_pending(&mut self, irq: usize) {
        if irq < self.lines.len() {
            self.lines[irq].pending = false;
        }
    }

    pub fn set_priority(&mut self, irq: usize, priority: u8) {
        if irq < self.lines.len() {
            self.lines[irq].priority = priority;
        }
    }

    /// Dispatches the highest-priority pending and enabled interrupt, if any.
    pub fn get_highest_pending_irq(&self) -> Option<usize> {
        if self.primask {
            return None;
        }

        let mut best_irq = None;
        let mut highest_prio = u8::MAX;

        for (irq, line) in self.lines.iter().enumerate() {
            #[allow(clippy::collapsible_if)]
            if line.enabled && line.pending && !line.active {
                if self.basepri == 0 || line.priority < self.basepri {
                    if line.priority < highest_prio {
                        highest_prio = line.priority;
                        best_irq = Some(irq);
                    }
                }
            }
        }

        best_irq
    }

    pub fn acknowledge_irq(&mut self, irq: usize) {
        if irq < self.lines.len() {
            self.lines[irq].pending = false;
            self.lines[irq].active = true;
        }
    }

    pub fn return_from_irq(&mut self, irq: usize) {
        if irq < self.lines.len() {
            self.lines[irq].active = false;
        }
    }
}
