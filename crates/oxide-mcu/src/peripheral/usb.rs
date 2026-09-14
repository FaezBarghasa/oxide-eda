//! Universal Serial Bus (USB 2.0 Device) Emulation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum UsbEndpointType {
    #[default]
    Control,
    Bulk,
    Interrupt,
    Isochronous,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsbEndpoint {
    pub number: u8,
    pub ep_type: UsbEndpointType,
    pub max_packet_size: u16,
    pub tx_buffer: Vec<u8>,
    pub rx_buffer: Vec<u8>,
    pub stalled: bool,
}

/// USB 2.0 Full-Speed / High-Speed Device Controller.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDevicePeripheral {
    pub name: String,
    pub enabled: bool,
    pub device_address: u8,
    pub configured: bool,
    pub endpoints: Vec<UsbEndpoint>,
}

impl UsbDevicePeripheral {
    pub fn new(name: impl Into<String>) -> Self {
        let mut endpoints = Vec::with_capacity(8);
        // EP0 Control Endpoint
        endpoints.push(UsbEndpoint {
            number: 0,
            ep_type: UsbEndpointType::Control,
            max_packet_size: 64,
            tx_buffer: Vec::new(),
            rx_buffer: Vec::new(),
            stalled: false,
        });

        Self {
            name: name.into(),
            enabled: false,
            device_address: 0,
            configured: false,
            endpoints,
        }
    }
}
