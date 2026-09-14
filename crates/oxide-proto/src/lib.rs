//! `oxide-proto` — IoT, Embedded & Network Protocol Simulation Engine for Oxide EDA.
//!
//! Provides:
//! - [`EmbeddedMqttBroker`]: Pure in-process, lightweight, zero-cloud MQTT 3.1.1/5.0 broker and subscriber tree inspector.
//! - [`VirtualEthernetBus`] & [`EthernetFrame`]: L2 packet switch and PCAP export for Wireshark analysis.
//! - [`VirtualWifiAp`] & [`WifiFrame`]: 802.11 beaconing, association, and free-space path loss (RSSI) calculation.
//! - [`VirtualBleDevice`] & [`BleAdvPacket`]: BLE HCI controller, GAP advertising, and GATT services/characteristics.

pub mod ble;
pub mod ethernet;
pub mod mqtt;
pub mod wifi;

pub use ble::{BleAdvPacket, BleAdvType, GattCharacteristic, GattService, VirtualBleDevice};
pub use ethernet::{EthernetFrame, VirtualEthernetBus};
pub use mqtt::{EmbeddedMqttBroker, MqttMessage, QosLevel};
pub use wifi::{VirtualWifiAp, WifiFrame, WifiFrameKind};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_mqtt_pubsub() {
        let mut broker = EmbeddedMqttBroker::new();
        broker.subscribe("client_ui", "sensor/+/temperature");

        let msg = MqttMessage {
            topic: "sensor/living_room/temperature".to_string(),
            payload: b"22.5".to_vec(),
            qos: QosLevel::AtLeastOnce,
            retain: false,
            timestamp_ms: 1000,
        };

        let recipients = broker.publish(msg.clone());
        assert_eq!(recipients, vec!["client_ui".to_string()]);
        assert_eq!(broker.message_log.len(), 1);
        assert_eq!(broker.message_log[0].payload_as_str(), "22.5");
    }

    #[test]
    fn test_ethernet_pcap_export() {
        let mut eth = VirtualEthernetBus::new();
        eth.transmit_frame(EthernetFrame {
            timestamp_us: 100,
            src_mac: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            dst_mac: [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
            ether_type: 0x0800, // IPv4
            payload: vec![1, 2, 3, 4],
        });

        let pcap_bytes = eth.export_pcap();
        assert!(pcap_bytes.len() > 24); // Contains PCAP global header + 1 packet
    }

    #[test]
    fn test_wifi_path_loss() {
        let ap = VirtualWifiAp::new("Oxide-Lab-AP", 1);
        let rssi_1m = ap.calculate_rssi_dbm(1.0);
        let rssi_10m = ap.calculate_rssi_dbm(10.0);
        // 10x distance in free space decreases signal by ~20 dB
        assert!((rssi_1m - rssi_10m - 20.0).abs() < 1.0);
    }
}
