---
okf_version: "0.2"
type: Function
title: editor_with_positions
description: "A wrapper editor pre-seeded with pads at the given world-mm centres,"
resource: crates/oxide-app/src/library/editor/footprint/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/tests/editor_with_positions
language: rust
---

# editor_with_positions

A wrapper editor pre-seeded with pads at the given world-mm centres,

## Signature

```rust
fn editor_with_positions(positions: &[(f64, f64)]) -> crate::app::FootprintEditorState
```

## Docstring

A wrapper editor pre-seeded with pads at the given world-mm centres,
the whole set selected (pad 0 primary + the rest as extras), reset to
a clean dirty/history baseline so a test asserts the *dispatcher* is
what moves pads / stacks history.

## Source
Lines 372–384 in `crates/oxide-app/src/library/editor/footprint/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/tests.md) |
| calls | [default_editor](/crates/oxide-app/src/library/editor/footprint/tests/default_editor.md) |
| called_by | [align_cancel_closes_modal_and_keeps_selection](/crates/oxide-app/src/library/editor/footprint/tests/align_cancel_closes_modal_and_keeps_selection.md) |
| called_by | [align_confirm_below_size_gate_pushes_no_history](/crates/oxide-app/src/library/editor/footprint/tests/align_confirm_below_size_gate_pushes_no_history.md) |
| called_by | [align_confirm_both_axes_is_one_undo_step](/crates/oxide-app/src/library/editor/footprint/tests/align_confirm_both_axes_is_one_undo_step.md) |
| called_by | [align_confirm_both_axes_matches_two_sequential_align_pads](/crates/oxide-app/src/library/editor/footprint/tests/align_confirm_both_axes_matches_two_sequential_align_pads.md) |
| called_by | [align_confirm_horizontal_matches_direct_align_pads](/crates/oxide-app/src/library/editor/footprint/tests/align_confirm_horizontal_matches_direct_align_pads.md) |
| called_by | [align_confirm_neither_axis_is_clean_noop](/crates/oxide-app/src/library/editor/footprint/tests/align_confirm_neither_axis_is_clean_noop.md) |
