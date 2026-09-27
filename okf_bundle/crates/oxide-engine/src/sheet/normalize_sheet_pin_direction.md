---
okf_version: "0.2"
type: Function
title: normalize_sheet_pin_direction
resource: crates/oxide-engine/src/sheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/sheet/normalize_sheet_pin_direction
language: rust
---

# normalize_sheet_pin_direction

## Signature

```rust
pub(crate) fn normalize_sheet_pin_direction(direction_or_shape: &str) -> &'static str
```

## Visibility

- `pub(crate)`

## Source
Lines 53–63 in `crates/oxide-engine/src/sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sheet](/crates/oxide-engine/src/sheet.md) |
| called_by | [collect_exposed_sheet_ports](/crates/oxide-engine/src/sheet/collect_exposed_sheet_ports.md) |
| called_by | [reconcile_child_sheet_pins](/crates/oxide-engine/src/sheet/reconcile_child_sheet_pins.md) |
