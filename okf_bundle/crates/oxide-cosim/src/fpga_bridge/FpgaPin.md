---
okf_version: "0.2"
type: Class
title: FpgaPin
description: FPGA Pin Definition.
resource: crates/oxide-cosim/src/fpga_bridge.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:44Z"
concept_id: crates/oxide-cosim/src/fpga_bridge/FpgaPin
language: rust
---

# FpgaPin

FPGA Pin Definition.

## Signature

```rust
pub struct FpgaPin
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

FPGA Pin Definition.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `state`
- `is_output`
- `tied_net`

## Source
Lines 17–22 in `crates/oxide-cosim/src/fpga_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fpga_bridge](/crates/oxide-cosim/src/fpga_bridge.md) |
