---
okf_version: "0.2"
type: Function
title: partition_cuttable
description: Splits a selection into the subset Cut can safely copy-then-delete and
resource: crates/oxide-engine/src/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/selection/partition_cuttable
language: rust
---

# partition_cuttable

Splits a selection into the subset Cut can safely copy-then-delete and

## Signature

```rust
pub fn partition_cuttable(items: &[SelectedItem]) -> (Vec<SelectedItem>, Vec<SelectedItem>)
```

## Visibility

- `pub`

## Docstring

Splits a selection into the subset Cut can safely copy-then-delete and
the remainder it must leave untouched. Cut is copy + delete; a kind
`collect_selection_clipboard` can't carry (`ChildSheet`, `SheetPin`,
`Drawing`, `BusEntry`, …) would otherwise get deleted with nothing in
the clipboard to restore it — a silent destroy, not a no-op (#341:
sheet clipboard support itself is out of scope, but Cut must not desync
from what Copy can actually carry).

## Source
Lines 630–635 in `crates/oxide-engine/src/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-engine/src/selection.md) |
| calls | [clipboard_can_carry](/crates/oxide-engine/src/selection/clipboard_can_carry.md) |
| called_by | [handle_selection_cut_requested](/crates/oxide-app/src/app/handlers/clipboard_workflows/handle_selection_cut_requested.md) |
| called_by | [partition_cuttable_keeps_child_sheet_and_pin_out_of_cut](/crates/oxide-engine/src/lib/partition_cuttable_keeps_child_sheet_and_pin_out_of_cut.md) |
