---
okf_version: "0.2"
type: Class
title: CanPeripheral
description: CAN Hardware peripheral model.
resource: crates/oxide-mcu/src/peripheral/can.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:02Z"
concept_id: crates/oxide-mcu/src/peripheral/can/CanPeripheral
language: rust
---

# CanPeripheral

CAN Hardware peripheral model.

## Signature

```rust
pub struct CanPeripheral
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

CAN Hardware peripheral model.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `enabled`
- `fd_enabled`
- `tx_mailboxes`
- `rx_fifo0`
- `rx_fifo1`
- `filter_banks`

## Source
Lines 32–40 in `crates/oxide-mcu/src/peripheral/can.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [can](/crates/oxide-mcu/src/peripheral/can.md) |
