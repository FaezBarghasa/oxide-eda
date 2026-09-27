---
okf_version: "0.2"
type: Function
title: pin_anchor_for_direction
resource: crates/oxide-engine/src/sheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/sheet/pin_anchor_for_direction
language: rust
---

# pin_anchor_for_direction

## Signature

```rust
fn pin_anchor_for_direction(child: &ChildSheet, direction: &str, slot: usize) -> (f64, f64, f64)
```

## Source
Lines 65–74 in `crates/oxide-engine/src/sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sheet](/crates/oxide-engine/src/sheet.md) |
| called_by | [reconcile_child_sheet_pins](/crates/oxide-engine/src/sheet/reconcile_child_sheet_pins.md) |
