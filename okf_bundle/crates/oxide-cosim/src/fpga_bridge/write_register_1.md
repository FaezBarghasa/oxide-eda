---
okf_version: "0.2"
type: Function
title: write_register
description: Write AXI/Wishbone register bus from MCU side.
resource: crates/oxide-cosim/src/fpga_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:44Z"
concept_id: crates/oxide-cosim/src/fpga_bridge/write_register_1
language: rust
---

# write_register

Write AXI/Wishbone register bus from MCU side.

## Signature

```rust
pub fn write_register(&mut self, offset: u32, val: u32)
```

## Visibility

- `pub`

## Docstring

Write AXI/Wishbone register bus from MCU side.

## Source
Lines 71–73 in `crates/oxide-cosim/src/fpga_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fpga_bridge](/crates/oxide-cosim/src/fpga_bridge.md) |
