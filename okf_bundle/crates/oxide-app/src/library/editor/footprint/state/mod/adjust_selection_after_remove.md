---
okf_version: "0.2"
type: Function
title: adjust_selection_after_remove
description: "HI-25 helper: when an item is removed at `removed_idx` from a Vec,"
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/adjust_selection_after_remove
language: rust
---

# adjust_selection_after_remove

HI-25 helper: when an item is removed at `removed_idx` from a Vec,

## Signature

```rust
pub(crate) fn adjust_selection_after_remove(
    selected: Option<usize>,
    removed_idx: usize,
) -> Option<usize>
```

## Visibility

- `pub(crate)`

## Docstring

HI-25 helper: when an item is removed at `removed_idx` from a Vec,
fold the change into a `selected: Option<usize>` so it still points
at the right element (or clears to `None` if the selection is what
got deleted). Used by the pad / silk / drawing deletion paths so
the "selection became dangling after delete" bug class can't recur.

- `None`                                 → `None`
- `Some(sel)` if `sel == removed_idx`    → `None`
- `Some(sel)` if `sel < removed_idx`     → `Some(sel)`
- `Some(sel)` if `sel > removed_idx`     → `Some(sel - 1)`

## Source
Lines 706–715 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
| called_by | [handle_fp_editor_delete_selected_silk](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/handle_fp_editor_delete_selected_silk.md) |
| called_by | [delete_pad](/crates/oxide-app/src/library/editor/footprint/state/mod/delete_pad.md) |
| called_by | [delete_silk_f](/crates/oxide-app/src/library/editor/footprint/updates/selection/delete_silk_f.md) |
