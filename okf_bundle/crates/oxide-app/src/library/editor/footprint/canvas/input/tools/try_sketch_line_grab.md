---
okf_version: "0.2"
type: Function
title: try_sketch_line_grab
description: "v0.27 — Fusion-style Line drag. In Sketch mode + Select tool, a"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_sketch_line_grab
language: rust
---

# try_sketch_line_grab

v0.27 — Fusion-style Line drag. In Sketch mode + Select tool, a

## Signature

```rust
impl FootprintCanvas<'_> { pub(in crate::library::editor::footprint::canvas) fn try_sketch_line_grab(
        &self,
        cstate: &mut FootprintCanvasState,
        cursor_pos: Point,
        world: (f64, f64),
    ) -> Option<canvas::Action<LibraryMessage>> }
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.27 — Fusion-style Line drag. In Sketch mode + Select tool, a
click within ~10 px of a Line's stroke (but missing the snap
radius for both endpoints) starts a Line-drag gesture: the
dispatcher translates BOTH endpoints by the per-tick delta in
one solver pass so an edge of a closed shape can be pushed
without having to grab a corner.

## Source
Lines 305–387 in `crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
