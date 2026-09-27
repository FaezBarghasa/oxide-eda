---
okf_version: "0.2"
type: Function
title: try_sketch_point_grab
description: v0.16 — Sketch mode + Select tool click within snap radius of a
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_sketch_point_grab
language: rust
---

# try_sketch_point_grab

v0.16 — Sketch mode + Select tool click within snap radius of a

## Signature

```rust
impl FootprintCanvas<'_> { pub(in crate::library::editor::footprint::canvas) fn try_sketch_point_grab(
        &self,
        cstate: &mut FootprintCanvasState,
        cursor_pos: Point,
        raw_world: (f64, f64),
        world: (f64, f64),
    ) -> Option<canvas::Action<LibraryMessage>> }
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.16 — Sketch mode + Select tool click within snap radius of a
sketch `Point` starts a Point-drag gesture and publishes a
select so the inspector + DOF overlay highlight immediately.

## Source
Lines 259–297 in `crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools.md) |
| calls | [sketch_snap](/crates/oxide-app/src/library/editor/footprint/canvas/hit_test/sketch_snap.md) |
