---
okf_version: "0.2"
type: Module
title: lib
description: "`oxide-proto` — IoT, Embedded & Network Protocol Simulation Engine for Oxide EDA."
resource: crates/oxide-proto/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:29:00Z"
concept_id: crates/oxide-proto/src/lib
language: rust
---

# lib

`oxide-proto` — IoT, Embedded & Network Protocol Simulation Engine for Oxide EDA.

## Docstring

`oxide-proto` — IoT, Embedded & Network Protocol Simulation Engine for Oxide EDA.

Provides:
- [`EmbeddedMqttBroker`]: Pure in-process, lightweight, zero-cloud MQTT 3.1.1/5.0 broker and subscriber tree inspector.
- [`VirtualEthernetBus`] & [`EthernetFrame`]: L2 packet switch and PCAP export for Wireshark analysis.
- [`VirtualWifiAp`] & [`WifiFrame`]: 802.11 beaconing, association, and free-space path loss (RSSI) calculation.
- [`VirtualBleDevice`] & [`BleAdvPacket`]: BLE HCI controller, GAP advertising, and GATT services/characteristics.

## Relationships

| Type | Target |
|------|--------|
| related | [test_embedded_mqtt_pubsub](/crates/oxide-proto/src/lib/test_embedded_mqtt_pubsub.md) |
| related | [test_ethernet_pcap_export](/crates/oxide-proto/src/lib/test_ethernet_pcap_export.md) |
| related | [test_wifi_path_loss](/crates/oxide-proto/src/lib/test_wifi_path_loss.md) |
