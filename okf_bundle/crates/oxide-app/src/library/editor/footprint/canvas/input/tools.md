---
okf_version: "0.2"
type: Module
title: tools
description: "Per-tool left-press gesture arms — the sequence of \"try to handle"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/tools
language: rust
---

# tools

Per-tool left-press gesture arms — the sequence of "try to handle

## Docstring

Per-tool left-press gesture arms — the sequence of "try to handle
this click" guards that the primary-press classifier walks in
order (lasso, touching-line, round-pad handle, sketch point / line
grabs, closed-loop select, pad grab, silk select).

Each `try_*` returns `Some(action)` when it claims the click and
`None` to fall through to the next handler — reproducing the
original top-to-bottom `if … { return … }` order byte-for-byte.

## Relationships

| Type | Target |
|------|--------|
| related | [try_lasso_add_vertex](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_lasso_add_vertex.md) |
| related | [try_touching_line_click](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_touching_line_click.md) |
| related | [try_round_handle_grab](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_round_handle_grab.md) |
| related | [try_drag_track_end_grab](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_drag_track_end_grab.md) |
| related | [try_sketch_point_grab](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_sketch_point_grab.md) |
| related | [try_sketch_line_grab](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_sketch_line_grab.md) |
| related | [try_closed_loop_select](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_closed_loop_select.md) |
| related | [try_pad_grab](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_pad_grab.md) |
| related | [try_silk_select](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_silk_select.md) |
| related | [try_lasso_add_vertex](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_lasso_add_vertex.md) |
| related | [try_touching_line_click](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_touching_line_click.md) |
| related | [try_round_handle_grab](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_round_handle_grab.md) |
| related | [try_drag_track_end_grab](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_drag_track_end_grab.md) |
| related | [try_sketch_point_grab](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_sketch_point_grab.md) |
| related | [try_sketch_line_grab](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_sketch_line_grab.md) |
| related | [try_closed_loop_select](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_closed_loop_select.md) |
| related | [try_pad_grab](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_pad_grab.md) |
| related | [try_silk_select](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_silk_select.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
