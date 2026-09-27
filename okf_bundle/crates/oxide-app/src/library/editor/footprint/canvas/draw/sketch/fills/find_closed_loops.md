---
okf_version: "0.2"
type: Function
title: find_closed_loops
description: v0.27 — find every closed loop in the sketch. Same adjacency
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/find_closed_loops
language: rust
---

# find_closed_loops

v0.27 — find every closed loop in the sketch. Same adjacency

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn find_closed_loops(
    sketch: &oxide_sketch::SketchData,
    state: &FootprintEditorState,
) -> Vec<ClosedLoop>
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.27 — find every closed loop in the sketch. Same adjacency
walk as the fill renderer; centralised so the click handler can
reuse it. Skips loops where every line is bake-skipped (purely
construction loops); those are visible only as dashed strokes
and selecting them via fill would surprise the user.

## Source
Lines 40–159 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fills](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [pos](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/pos.md) |
| called_by | [try_closed_loop_select](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_closed_loop_select.md) |
