//! Direct Memory Access (DMA) Controller Emulation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DmaDirection {
    #[default]
    PeripheralToMemory,
    MemoryToPeripheral,
    MemoryToMemory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DmaPriority {
    Low,
    #[default]
    Medium,
    High,
    VeryHigh,
}

/// A single DMA Stream / Channel.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DmaChannel {
    pub enabled: bool,
    pub direction: DmaDirection,
    pub priority: DmaPriority,
    pub circular_mode: bool,
    pub source_addr: u32,
    pub dest_addr: u32,
    pub total_items: usize,
    pub current_items_remaining: usize,
    pub half_transfer_interrupt: bool,
    pub transfer_complete_interrupt: bool,
    pub error_interrupt: bool,
    pub transfer_complete: bool,
    pub half_transfer_complete: bool,
}

/// Multi-channel DMA Controller Model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmaController {
    pub name: String,
    pub channels: Vec<DmaChannel>,
}

impl DmaController {
    pub fn new(name: impl Into<String>, num_channels: usize) -> Self {
        Self {
            name: name.into(),
            channels: vec![DmaChannel::default(); num_channels],
        }
    }

    pub fn start_transfer(&mut self, channel_idx: usize, src: u32, dst: u32, count: usize) {
        if let Some(ch) = self.channels.get_mut(channel_idx) {
            ch.enabled = true;
            ch.source_addr = src;
            ch.dest_addr = dst;
            ch.total_items = count;
            ch.current_items_remaining = count;
            ch.transfer_complete = false;
            ch.half_transfer_complete = false;
        }
    }

    /// Steps DMA transfers by transferring N items.
    pub fn step_transfers(&mut self, items_to_transfer: usize) {
        for ch in &mut self.channels {
            if ch.enabled && ch.current_items_remaining > 0 {
                let trans = items_to_transfer.min(ch.current_items_remaining);
                ch.current_items_remaining -= trans;

                // Check half-transfer
                if ch.current_items_remaining <= (ch.total_items / 2) && !ch.half_transfer_complete {
                    ch.half_transfer_complete = true;
                }

                // Check transfer complete
                if ch.current_items_remaining == 0 {
                    ch.transfer_complete = true;
                    if ch.circular_mode {
                        ch.current_items_remaining = ch.total_items;
                        ch.half_transfer_complete = false;
                    } else {
                        ch.enabled = false;
                    }
                }
            }
        }
    }
}
