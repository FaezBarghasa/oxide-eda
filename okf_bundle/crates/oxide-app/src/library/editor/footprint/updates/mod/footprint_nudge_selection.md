---
okf_version: "0.2"
type: Function
title: footprint_nudge_selection
description: "Translate the current pad selection by (dx, dy) mm: history"
resource: crates/oxide-app/src/library/editor/footprint/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/mod/footprint_nudge_selection
language: rust
---

# footprint_nudge_selection

Translate the current pad selection by (dx, dy) mm: history

## Signature

```rust
fn footprint_nudge_selection(editor: &mut crate::app::FootprintEditorState, dx: f64, dy: f64)
```

## Docstring

Translate the current pad selection by (dx, dy) mm: history
snapshot, tested `nudge_pads`, sketch mirror, primitive re-sync.
No-op on an empty selection. Shared by the one-step
`FootprintActiveBarNudgeSelection` nudge and the typed-delta
Move-By modal (`FootprintMoveByConfirm`) so both paths share the
exact same proven geometry + sketch-mirror + history behaviour.

## Source
Lines 452–478 in `crates/oxide-app/src/library/editor/footprint/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/footprint/updates/mod.md) |
| calls | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
| called_by | [active_bar_nudge_selection](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_nudge_selection.md) |
| called_by | [move_by_confirm](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/move_by_confirm.md) |
