---
okf_version: "0.2"
type: Function
title: trim
description: "v0.27 — EDA Trim. Single click on a Line: find its self-intersections"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/trim
language: rust
---

# trim

v0.27 — EDA Trim. Single click on a Line: find its self-intersections

## Signature

```rust
fn trim(editor: &mut crate::app::FootprintEditorState, ctx: &ToolClickCtx)
```

## Docstring

v0.27 — EDA Trim. Single click on a Line: find its self-intersections
with all other Lines, pick the two intersections that bracket the
click point on the line, split the line into up-to-three segments,
and remove the middle segment containing the click. If only one
intersection exists, remove the side containing the click. If no
intersection exists, remove the whole Line (Fusion-style "trim to
nothing" is a useful EDA fallback for stripping a stray overlap).

## Source
Lines 377–641 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [edit](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [line_xy](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/line_xy.md) |
| calls | [pick_line_at_for_trim](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/pick_line_at_for_trim.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/apply.md) |
