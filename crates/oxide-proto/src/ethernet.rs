//! Virtual Ethernet Switch, Packet Forwarding & PCAP Stream Capture.

use serde::{Deserialize, Serialize};

/// Captured Ethernet Packet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EthernetFrame {
    pub timestamp_us: u64,
    pub src_mac: [u8; 6],
    pub dst_mac: [u8; 6],
    pub ether_type: u16,
    pub payload: Vec<u8>,
}

/// Virtual L2 Ethernet Network Bus & Packet Sniffer.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VirtualEthernetBus {
    pub captured_frames: Vec<EthernetFrame>,
}

impl VirtualEthernetBus {
    pub fn new() -> Self {
        Self::default()
    }

    /// Broadcasts or unicasts an Ethernet frame across connected simulated devices.
    pub fn transmit_frame(&mut self, frame: EthernetFrame) {
        self.captured_frames.push(frame);
    }

    /// Generates standard libpcap binary file header and recorded packet stream.
    pub fn export_pcap(&self) -> Vec<u8> {
        let mut pcap = Vec::new();

        // Global Header (24 bytes)
        pcap.extend_from_slice(&0xa1b2c3d4u32.to_le_bytes()); // Magic number
        pcap.extend_from_slice(&2u16.to_le_bytes());          // Major version
        pcap.extend_from_slice(&4u16.to_le_bytes());          // Minor version
        pcap.extend_from_slice(&0u32.to_le_bytes());          // Thiszone (GMT)
        pcap.extend_from_slice(&0u32.to_le_bytes());          // Sigfigs
        pcap.extend_from_slice(&65535u32.to_le_bytes());      // Snaplen
        pcap.extend_from_slice(&1u32.to_le_bytes());          // Network: Ethernet (1)

        for f in &self.captured_frames {
            let mut raw_pkt = Vec::with_capacity(14 + f.payload.len());
            raw_pkt.extend_from_slice(&f.dst_mac);
            raw_pkt.extend_from_slice(&f.src_mac);
            raw_pkt.extend_from_slice(&f.ether_type.to_be_bytes());
            raw_pkt.extend_from_slice(&f.payload);

            let len = raw_pkt.len() as u32;
            let ts_sec = (f.timestamp_us / 1_000_000) as u32;
            let ts_usec = (f.timestamp_us % 1_000_000) as u32;

            // Packet Header (16 bytes)
            pcap.extend_from_slice(&ts_sec.to_le_bytes());
            pcap.extend_from_slice(&ts_usec.to_le_bytes());
            pcap.extend_from_slice(&len.to_le_bytes());
            pcap.extend_from_slice(&len.to_le_bytes());
            pcap.extend_from_slice(&raw_pkt);
        }

        pcap
    }

    pub fn clear(&mut self) {
        self.captured_frames.clear();
    }
}
