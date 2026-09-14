//! Controller Area Network (CAN 2.0A/B & CAN-FD) Emulation.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanFrame {
    pub id: u32,
    pub is_extended_id: bool,
    pub is_fd: bool,
    pub is_rtr: bool,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CanFilterMode {
    #[default]
    IdMask,
    IdList,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanFilterBank {
    pub enabled: bool,
    pub mode: CanFilterMode,
    pub id: u32,
    pub mask: u32,
}

/// CAN Hardware peripheral model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanPeripheral {
    pub name: String,
    pub enabled: bool,
    pub fd_enabled: bool,
    pub tx_mailboxes: VecDeque<CanFrame>,
    pub rx_fifo0: VecDeque<CanFrame>,
    pub rx_fifo1: VecDeque<CanFrame>,
    pub filter_banks: Vec<CanFilterBank>,
}

impl CanPeripheral {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            enabled: true,
            fd_enabled: true,
            tx_mailboxes: VecDeque::with_capacity(3),
            rx_fifo0: VecDeque::with_capacity(3),
            rx_fifo1: VecDeque::with_capacity(3),
            filter_banks: vec![CanFilterBank::default(); 14],
        }
    }

    pub fn transmit(&mut self, frame: CanFrame) -> bool {
        if self.tx_mailboxes.len() < 3 {
            self.tx_mailboxes.push_back(frame);
            true
        } else {
            false
        }
    }

    pub fn receive_into_fifo0(&mut self, frame: CanFrame) -> bool {
        if self.rx_fifo0.len() < 3 {
            self.rx_fifo0.push_back(frame);
            true
        } else {
            false
        }
    }
}
