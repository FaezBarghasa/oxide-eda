---
okf_version: "0.2"
type: Function
title: send_address
resource: crates/oxide-mcu/src/peripheral/i2c.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:51Z"
concept_id: crates/oxide-mcu/src/peripheral/i2c/send_address_1
language: rust
---

# send_address

## Signature

```rust
pub fn send_address(&mut self, addr: u16, is_read: bool) -> bool
```

## Visibility

- `pub`

## Source
Lines 56–61 in `crates/oxide-mcu/src/peripheral/i2c.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [i2c](/crates/oxide-mcu/src/peripheral/i2c.md) |
