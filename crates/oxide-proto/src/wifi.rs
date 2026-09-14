//! Virtual 802.11 Wi-Fi Link & Physical Path-Loss Simulator.

use serde::{Deserialize, Serialize};

/// 802.11 Frame Subtype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WifiFrameKind {
    Beacon,
    ProbeRequest,
    ProbeResponse,
    Authentication,
    AssociationRequest,
    AssociationResponse,
    Data,
}

/// Simulated 802.11 Wi-Fi Frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WifiFrame {
    pub kind: WifiFrameKind,
    pub ssid: String,
    pub bssid: [u8; 6],
    pub client_mac: [u8; 6],
    pub channel: u8,
    pub rssi_dbm: f64,
    pub payload: Vec<u8>,
}

/// Virtual Wi-Fi Access Point (AP) state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualWifiAp {
    pub ssid: String,
    pub bssid: [u8; 6],
    pub channel: u8,
    pub tx_power_dbm: f64,
    pub connected_stations: Vec<[u8; 6]>,
}

impl VirtualWifiAp {
    pub fn new(ssid: impl Into<String>, channel: u8) -> Self {
        Self {
            ssid: ssid.into(),
            bssid: [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E],
            channel,
            tx_power_dbm: 20.0,
            connected_stations: Vec::new(),
        }
    }

    /// Calculates received signal strength (RSSI) based on free-space path loss (FSPL).
    /// $FSPL(dB) = 20 \log_{10}(d) + 20 \log_{10}(f) - 147.55$
    pub fn calculate_rssi_dbm(&self, distance_meters: f64) -> f64 {
        let d = distance_meters.max(0.1);
        let freq_hz: f64 = 2.412e9; // 2.4 GHz Channel 1
        let fspl_db = 20.0 * d.log10() + 20.0 * freq_hz.log10() - 147.55;
        self.tx_power_dbm - fspl_db
    }

    /// Emits 802.11 Beacon frame.
    pub fn emit_beacon(&self, distance_meters: f64) -> WifiFrame {
        WifiFrame {
            kind: WifiFrameKind::Beacon,
            ssid: self.ssid.clone(),
            bssid: self.bssid,
            client_mac: [0xFF; 6], // Broadcast
            channel: self.channel,
            rssi_dbm: self.calculate_rssi_dbm(distance_meters),
            payload: Vec::new(),
        }
    }
}
