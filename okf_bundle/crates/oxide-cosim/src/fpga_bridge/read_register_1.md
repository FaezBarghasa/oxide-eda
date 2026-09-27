---
okf_version: "0.2"
type: Function
title: read_register
description: Read AXI/Wishbone register bus from MCU side.
resource: crates/oxide-cosim/src/fpga_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:44Z"
concept_id: crates/oxide-cosim/src/fpga_bridge/read_register_1
language: rust
---

# read_register

Read AXI/Wishbone register bus from MCU side.

## Signature

```rust
pub fn read_register(&self, offset: u32) -> u32
```

## Visibility

- `pub`

## Docstring

Read AXI/Wishbone register bus from MCU side.

## Source
Lines 66–68 in `crates/oxide-cosim/src/fpga_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fpga_bridge](/crates/oxide-cosim/src/fpga_bridge.md) |
