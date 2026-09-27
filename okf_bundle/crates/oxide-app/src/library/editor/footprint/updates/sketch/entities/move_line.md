---
okf_version: "0.2"
type: Function
title: move_line
description: v0.27 — drag a Line edge by translating both its endpoints in one
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/move_line
language: rust
---

# move_line

v0.27 — drag a Line edge by translating both its endpoints in one

## Signature

```rust
fn move_line(
    editor: &mut crate::app::FootprintEditorState,
    id: oxide_sketch::id::SketchEntityId,
    dx: f64,
    dy: f64,
)
```

## Docstring

v0.27 — drag a Line edge by translating both its endpoints in one
solver pass. The dispatcher reads the Line's start/end IDs, then emits
MovePoint for each. The solver re-runs once after both moves so H/V/
Distance constraints converge correctly without the brief mid-tick
"one corner moved, the other didn't" state a two-message split would
produce.

## Source
Lines 194–400 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [entities](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| calls | [propagate_line_drag_to_pad_bboxes](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/propagate_line_drag_to_pad_bboxes.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/apply.md) |
