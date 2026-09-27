---
okf_version: "0.2"
type: Function
title: test_child_sheet
resource: crates/oxide-engine/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T12:59:13Z"
concept_id: crates/oxide-engine/src/lib/test_child_sheet
language: rust
---

# test_child_sheet

## Signature

```rust
fn test_child_sheet(pins: Vec<SheetPin>) -> ChildSheet
```

## Source
Lines 386–401 in `crates/oxide-engine/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-engine/src/lib.md) |
| called_by | [delete_selection_of_child_sheet_and_sheet_pin_are_undoable](/crates/oxide-engine/src/lib/delete_selection_of_child_sheet_and_sheet_pin_are_undoable.md) |
| called_by | [delete_selection_removes_child_sheet_and_its_pins](/crates/oxide-engine/src/lib/delete_selection_removes_child_sheet_and_its_pins.md) |
| called_by | [delete_selection_removes_only_the_targeted_sheet_pin](/crates/oxide-engine/src/lib/delete_selection_removes_only_the_targeted_sheet_pin.md) |
| called_by | [deleting_a_port_backed_sheet_pin_does_not_survive_reconcile](/crates/oxide-engine/src/lib/deleting_a_port_backed_sheet_pin_does_not_survive_reconcile.md) |
| called_by | [has_selected_items_recognizes_child_sheet_and_pin](/crates/oxide-engine/src/lib/has_selected_items_recognizes_child_sheet_and_pin.md) |
