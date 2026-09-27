---
okf_version: "0.2"
type: Function
title: try_lasso_add_vertex
description: v0.27 — Lasso Select intercept. While the lasso tool is armed
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_lasso_add_vertex_1
language: rust
---

# try_lasso_add_vertex

v0.27 — Lasso Select intercept. While the lasso tool is armed

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn try_lasso_add_vertex(
        &self,
        cstate: &FootprintCanvasState,
        cursor_pos: Point,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.27 — Lasso Select intercept. While the lasso tool is armed
(set from the active-bar Selection Mode dropdown), each
left-click adds a vertex to the in-flight polygon. Right-click
commits / Esc cancels are handled in their own arms.

## Source
Lines 27–48 in `crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools.md) |
