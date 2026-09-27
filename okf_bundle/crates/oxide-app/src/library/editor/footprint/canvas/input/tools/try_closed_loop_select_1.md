---
okf_version: "0.2"
type: Function
title: try_closed_loop_select
description: "v0.27 — Fusion-style \"click the fill, select the closed shape.\""
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_closed_loop_select_1
language: rust
---

# try_closed_loop_select

v0.27 — Fusion-style "click the fill, select the closed shape."

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn try_closed_loop_select(
        &self,
        world: (f64, f64),
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.27 — Fusion-style "click the fill, select the closed shape."
Only in Sketch mode + Select tool, and only when the
Point-snap path missed. Walks the same closed-loop adjacency
the fill renderer uses and dispatches a SelectMany carrying
every Line + Point in the loop.

## Source
Lines 394–425 in `crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools.md) |
| calls | [find_closed_loops](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/find_closed_loops.md) |
