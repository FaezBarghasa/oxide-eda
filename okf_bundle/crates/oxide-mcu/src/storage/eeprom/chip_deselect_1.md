---
okf_version: "0.2"
type: Function
title: chip_deselect
description: "Pull CS High: Ends transaction and commits page writes."
resource: crates/oxide-mcu/src/storage/eeprom.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:00Z"
concept_id: crates/oxide-mcu/src/storage/eeprom/chip_deselect_1
language: rust
---

# chip_deselect

Pull CS High: Ends transaction and commits page writes.

## Signature

```rust
pub fn chip_deselect(&mut self)
```

## Visibility

- `pub`

## Docstring

Pull CS High: Ends transaction and commits page writes.

## Source
Lines 132–135 in `crates/oxide-mcu/src/storage/eeprom.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eeprom](/crates/oxide-mcu/src/storage/eeprom.md) |
