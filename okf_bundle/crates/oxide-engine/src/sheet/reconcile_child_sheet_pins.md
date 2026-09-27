---
okf_version: "0.2"
type: Function
title: reconcile_child_sheet_pins
resource: crates/oxide-engine/src/sheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/sheet/reconcile_child_sheet_pins
language: rust
---

# reconcile_child_sheet_pins

## Signature

```rust
pub(crate) fn reconcile_child_sheet_pins(child: &mut ChildSheet, ports: &[SheetPort]) -> bool
```

## Visibility

- `pub(crate)`

## Source
Lines 154–221 in `crates/oxide-engine/src/sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sheet](/crates/oxide-engine/src/sheet.md) |
| calls | [normalize_sheet_pin_direction](/crates/oxide-engine/src/sheet/normalize_sheet_pin_direction.md) |
| calls | [pin_anchor_for_direction](/crates/oxide-engine/src/sheet/pin_anchor_for_direction.md) |
| calls | [same_sheet_pin](/crates/oxide-engine/src/sheet/same_sheet_pin.md) |
| called_by | [exec_structure](/crates/oxide-engine/src/exec/structure/exec_structure.md) |
| called_by | [deleting_a_port_backed_sheet_pin_does_not_survive_reconcile](/crates/oxide-engine/src/lib/deleting_a_port_backed_sheet_pin_does_not_survive_reconcile.md) |
