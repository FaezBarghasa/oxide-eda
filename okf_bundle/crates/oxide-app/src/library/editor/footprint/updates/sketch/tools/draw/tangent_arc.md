---
okf_version: "0.2"
type: Function
title: tangent_arc
description: v0.24 Track C — Tangent Arc. Two-click chained arc segment that mints
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/tangent_arc
language: rust
---

# tangent_arc

v0.24 Track C — Tangent Arc. Two-click chained arc segment that mints

## Signature

```rust
fn tangent_arc(editor: &mut crate::app::FootprintEditorState, ctx: &ToolClickCtx)
```

## Docstring

v0.24 Track C — Tangent Arc. Two-click chained arc segment that mints
an Arc tangent to the most recently committed Line whose end Point
matches the first click. The dispatcher also emits a `TangentLineArc`
constraint so the tangency survives further edits.

- Click 1: stash the resolved Point as
`ToolPending::TangentArcFirst { first }`. Mirrors the Line tool's
first-click flow.
- Click 2: locate a Line whose `end == first`. Compute the tangent
centre on the line's perpendicular bisector through `first` so the
arc starts off the line tangentially. Mint an Arc entity +
TangentLineArc constraint and chain back to Idle.

Fallback: when no incident Line is found, the dispatcher mints a
placeholder centre at the perpendicular bisector of the chord (no
tangency reference) and publishes a warning via `solve_warnings`. The
Arc still appears in the sketch so the user can constrain it manually
if desired.

## Source
Lines 594–813 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [draw](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/apply.md) |
