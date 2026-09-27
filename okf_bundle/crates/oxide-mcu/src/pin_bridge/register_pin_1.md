---
okf_version: "0.2"
type: Function
title: register_pin
resource: crates/oxide-mcu/src/pin_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:51Z"
concept_id: crates/oxide-mcu/src/pin_bridge/register_pin_1
language: rust
---

# register_pin

## Signature

```rust
pub fn register_pin(&mut self, pin_name: impl Into<String>, function: PinFunction, connected_net: Option<String>)
```

## Visibility

- `pub`

## Source
Lines 109–114 in `crates/oxide-mcu/src/pin_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin_bridge](/crates/oxide-mcu/src/pin_bridge.md) |
