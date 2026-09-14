//! 10/100M Ethernet MAC Emulation.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthernetFrame {
    pub dest_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub ethertype: u16,
    pub payload: Vec<u8>,
}

/// 10/100M Ethernet MAC Hardware peripheral model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthernetMacPeripheral {
    pub name: String,
    pub enabled: bool,
    pub mac_address: [u8; 6],
    pub tx_descriptors: VecDeque<EthernetFrame>,
    pub rx_descriptors: VecDeque<EthernetFrame>,
    pub link_up: bool,
}

impl EthernetMacPeripheral {
    pub fn new(name: impl Into<String>, mac: [u8; 6]) -> Self {
        Self {
            name: name.into(),
            enabled: true,
            mac_address: mac,
            tx_descriptors: VecDeque::with_capacity(16),
            rx_descriptors: VecDeque::with_capacity(16),
            link_up: true,
        }
    }

    pub fn queue_tx_frame(&mut self, frame: EthernetFrame) {
        self.tx_descriptors.push_back(frame);
    }

    pub fn receive_rx_frame(&mut self, frame: EthernetFrame) {
        self.rx_descriptors.push_back(frame);
    }
}
