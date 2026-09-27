---
okf_version: "0.2"
type: Class
title: FpgaBridge
description: Cycle-accurate FPGA Simulation Bridge.
resource: crates/oxide-cosim/src/fpga_bridge.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:44Z"
concept_id: crates/oxide-cosim/src/fpga_bridge/FpgaBridge
language: rust
---

# FpgaBridge

Cycle-accurate FPGA Simulation Bridge.

## Signature

```rust
pub struct FpgaBridge
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Cycle-accurate FPGA Simulation Bridge.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `target_device`
- `clock_freq_hz`
- `current_cycle`
- `pins`
- `memory_mapped_regs`

## Source
Lines 26–33 in `crates/oxide-cosim/src/fpga_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fpga_bridge](/crates/oxide-cosim/src/fpga_bridge.md) |
