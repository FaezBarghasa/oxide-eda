//! Bluetooth Low Energy (BLE) Host Controller Interface (HCI) & GATT Simulator.

use serde::{Deserialize, Serialize};

/// BLE Advertising Packet Type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BleAdvType {
    AdvInd,        // Connectable Undirected
    AdvDirectInd,  // Connectable Directed
    AdvNonconnInd, // Non-connectable Undirected
    AdvScanInd,    // Scannable Undirected
}

/// Simulated BLE Advertising Payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BleAdvPacket {
    pub adv_type: BleAdvType,
    pub address: [u8; 6],
    pub device_name: Option<String>,
    pub service_uuids: Vec<u16>,
    pub rssi_dbm: i8,
    pub raw_data: Vec<u8>,
}

/// BLE GATT Characteristic Definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GattCharacteristic {
    pub uuid: u16,
    pub properties: u8, // Read (0x02), Write (0x08), Notify (0x10)
    pub value: Vec<u8>,
}

/// BLE GATT Service Definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GattService {
    pub uuid: u16,
    pub characteristics: Vec<GattCharacteristic>,
}

/// Virtual BLE Device State.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualBleDevice {
    pub address: [u8; 6],
    pub device_name: String,
    pub is_advertising: bool,
    pub services: Vec<GattService>,
}

impl VirtualBleDevice {
    pub fn new(device_name: impl Into<String>) -> Self {
        Self {
            address: [0xC0, 0xDE, 0x00, 0x11, 0x22, 0x33],
            device_name: device_name.into(),
            is_advertising: true,
            services: Vec::new(),
        }
    }

    pub fn add_service(&mut self, service: GattService) {
        self.services.push(service);
    }

    /// Emits GAP advertising payload packet.
    pub fn emit_adv_packet(&self, rssi_dbm: i8) -> BleAdvPacket {
        let uuids: Vec<u16> = self.services.iter().map(|s| s.uuid).collect();
        BleAdvPacket {
            adv_type: BleAdvType::AdvInd,
            address: self.address,
            device_name: Some(self.device_name.clone()),
            service_uuids: uuids,
            rssi_dbm,
            raw_data: Vec::new(),
        }
    }
}
