---
okf_version: "0.2"
type: Class
title: ComparatorPeripheral
description: Analog Comparator Model.
resource: crates/oxide-mcu/src/peripheral/comparator.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:32Z"
concept_id: crates/oxide-mcu/src/peripheral/comparator/ComparatorPeripheral
language: rust
---

# ComparatorPeripheral

Analog Comparator Model.

## Signature

```rust
pub struct ComparatorPeripheral
```

## Decorators

- `derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Analog Comparator Model.
[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]

## Methods

- `name`
- `enabled`
- `hysteresis`
- `polarity`
- `v_in_pos`
- `v_in_neg`
- `output_state`

## Source
Lines 23–31 in `crates/oxide-mcu/src/peripheral/comparator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [comparator](/crates/oxide-mcu/src/peripheral/comparator.md) |
