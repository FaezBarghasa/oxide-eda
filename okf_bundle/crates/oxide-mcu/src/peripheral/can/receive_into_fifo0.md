---
okf_version: "0.2"
type: Function
title: receive_into_fifo0
resource: crates/oxide-mcu/src/peripheral/can.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:02Z"
concept_id: crates/oxide-mcu/src/peripheral/can/receive_into_fifo0
language: rust
---

# receive_into_fifo0

## Signature

```rust
impl CanPeripheral { pub fn receive_into_fifo0(&mut self, frame: CanFrame) -> bool }
```

## Visibility

- `pub`

## Source
Lines 64–71 in `crates/oxide-mcu/src/peripheral/can.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [can](/crates/oxide-mcu/src/peripheral/can.md) |
