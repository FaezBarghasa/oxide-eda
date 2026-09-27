---
okf_version: "0.2"
type: Function
title: try_round_handle_grab
description: "v0.27 — Sketch mode + Select tool: hit-test the east-edge cyan"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_round_handle_grab
language: rust
---

# try_round_handle_grab

v0.27 — Sketch mode + Select tool: hit-test the east-edge cyan

## Signature

```rust
impl FootprintCanvas<'_> { pub(in crate::library::editor::footprint::canvas) fn try_round_handle_grab(
        &self,
        cstate: &mut FootprintCanvasState,
        cursor_pos: Point,
        world: (f64, f64),
    ) -> Option<canvas::Action<LibraryMessage>> }
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.27 — Sketch mode + Select tool: hit-test the east-edge cyan
diameter handle of any Round pad before the generic Point-snap.
The handle is drawn at (pad.position + (pad.size_x/2, 0)) with
a 4 px radius, so allow a 6 px hit slop.

## Source
Lines 89–126 in `crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools.md) |
