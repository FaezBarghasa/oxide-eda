---
okf_version: "0.2"
type: Function
title: horizontal_edge_at_y
description: "The `Line` whose two endpoints both sit at world y == `y` — a pad's"
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint/horizontal_edge_at_y
language: rust
---

# horizontal_edge_at_y

The `Line` whose two endpoints both sit at world y == `y` — a pad's

## Signature

```rust
fn horizontal_edge_at_y(app: &Oxide, path: &PathBuf, y: f64) -> oxide_sketch::id::SketchEntityId
```

## Docstring

The `Line` whose two endpoints both sit at world y == `y` — a pad's
top or bottom bbox edge, the handle a Sketch-mode edge drag grabs.

## Source
Lines 483–511 in `crates/oxide-app/tests/footprint_pad_remint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_remint](/crates/oxide-app/tests/footprint_pad_remint.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [pos](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/pos.md) |
| called_by | [a_streamed_sketch_edge_drag_keeps_resizing_after_the_first_tick](/crates/oxide-app/tests/footprint_pad_remint/a_streamed_sketch_edge_drag_keeps_resizing_after_the_first_tick.md) |
| called_by | [sketch_edge_drag_regenerates_the_chamfer_anchor](/crates/oxide-app/tests/footprint_pad_remint/sketch_edge_drag_regenerates_the_chamfer_anchor.md) |
