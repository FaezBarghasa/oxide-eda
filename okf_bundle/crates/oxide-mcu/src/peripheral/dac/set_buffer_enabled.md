---
okf_version: "0.2"
type: Function
title: set_buffer_enabled
resource: crates/oxide-mcu/src/peripheral/dac.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:24Z"
concept_id: crates/oxide-mcu/src/peripheral/dac/set_buffer_enabled
language: rust
---

# set_buffer_enabled

## Signature

```rust
impl DacPeripheral { pub fn set_buffer_enabled(&mut self, channel_idx: usize, enabled: bool) }
```

## Visibility

- `pub`

## Source
Lines 59–65 in `crates/oxide-mcu/src/peripheral/dac.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dac](/crates/oxide-mcu/src/peripheral/dac.md) |
