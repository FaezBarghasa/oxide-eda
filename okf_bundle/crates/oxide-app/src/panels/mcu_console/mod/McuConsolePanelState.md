---
okf_version: "0.2"
type: Class
title: McuConsolePanelState
description: "State for the MCU & Protocol Console Panel."
resource: crates/oxide-app/src/panels/mcu_console/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T09:20:06Z"
concept_id: crates/oxide-app/src/panels/mcu_console/mod/McuConsolePanelState
language: rust
---

# McuConsolePanelState

State for the MCU & Protocol Console Panel.

## Signature

```rust
pub struct McuConsolePanelState
```

## Decorators

- `derive(Debug, Clone, Default)`

## Visibility

- `pub`

## Docstring

State for the MCU & Protocol Console Panel.
[derive(Debug, Clone, Default)]

## Methods

- `active_tab`
- `uart_output`
- `uart_input_buffer`
- `mqtt_messages`
- `is_qemu_running`
- `gdb_port`
- `cosim_time_us`
- `eth_packet_count`
- `wifi_rssi_dbm`
- `ble_connected`

## Source
Lines 23–34 in `crates/oxide-app/src/panels/mcu_console/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcu_console](/crates/oxide-app/src/panels/mcu_console/mod.md) |
