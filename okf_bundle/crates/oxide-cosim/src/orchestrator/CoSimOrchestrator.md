---
okf_version: "0.2"
type: Class
title: CoSimOrchestrator
description: "Master Co-Simulation Orchestrator tying SPICE analog, QEMU MCU, and virtual network protocol engines."
resource: crates/oxide-cosim/src/orchestrator.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:33:10Z"
concept_id: crates/oxide-cosim/src/orchestrator/CoSimOrchestrator
language: rust
---

# CoSimOrchestrator

Master Co-Simulation Orchestrator tying SPICE analog, QEMU MCU, and virtual network protocol engines.

## Signature

```rust
pub struct CoSimOrchestrator
```

## Visibility

- `pub`

## Docstring

Master Co-Simulation Orchestrator tying SPICE analog, QEMU MCU, and virtual network protocol engines.

## Methods

- `status`
- `stats`
- `step_time_s`
- `pin_bridge`
- `virtual_uart`
- `mqtt_broker`
- `ethernet_bus`
- `analog_dataset`

## Source
Lines 12–21 in `crates/oxide-cosim/src/orchestrator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [orchestrator](/crates/oxide-cosim/src/orchestrator.md) |
