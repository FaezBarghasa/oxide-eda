---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-mcu/src/pin_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:51Z"
concept_id: crates/oxide-mcu/src/pin_bridge/new
language: rust
---

# new

## Signature

```rust
impl VirtualPinState { pub fn new(pin_name: impl Into<String>, function: PinFunction) -> Self }
```

## Visibility

- `pub`

## Source
Lines 80–89 in `crates/oxide-mcu/src/pin_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin_bridge](/crates/oxide-mcu/src/pin_bridge.md) |
