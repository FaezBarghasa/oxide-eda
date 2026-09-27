---
okf_version: "0.2"
type: Function
title: register_pin
description: Register a named FPGA pin.
resource: crates/oxide-cosim/src/fpga_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:44Z"
concept_id: crates/oxide-cosim/src/fpga_bridge/register_pin
language: rust
---

# register_pin

Register a named FPGA pin.

## Signature

```rust
impl FpgaBridge { pub fn register_pin(&mut self, name: &str, is_output: bool, tied_net: Option<String>) }
```

## Visibility

- `pub`

## Docstring

Register a named FPGA pin.

## Source
Lines 48–58 in `crates/oxide-cosim/src/fpga_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fpga_bridge](/crates/oxide-cosim/src/fpga_bridge.md) |
