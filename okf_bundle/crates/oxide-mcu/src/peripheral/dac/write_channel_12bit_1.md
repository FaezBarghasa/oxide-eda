---
okf_version: "0.2"
type: Function
title: write_channel_12bit
resource: crates/oxide-mcu/src/peripheral/dac.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:24Z"
concept_id: crates/oxide-mcu/src/peripheral/dac/write_channel_12bit_1
language: rust
---

# write_channel_12bit

## Signature

```rust
pub fn write_channel_12bit(&mut self, channel_idx: usize, data_12bit: u16)
```

## Visibility

- `pub`

## Source
Lines 49–57 in `crates/oxide-mcu/src/peripheral/dac.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dac](/crates/oxide-mcu/src/peripheral/dac.md) |
