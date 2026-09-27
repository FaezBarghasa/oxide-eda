---
okf_version: "0.2"
type: Function
title: sketch_hit_other
description: "v0.13.3 — Hit-test Lines / Arcs / Circles (everything that isn't"
resource: crates/oxide-app/src/library/editor/footprint/canvas/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/hit_test/sketch_hit_other
language: rust
---

# sketch_hit_other

v0.13.3 — Hit-test Lines / Arcs / Circles (everything that isn't

## Signature

```rust
pub(super) fn sketch_hit_other(
    sketch: Option<&oxide_sketch::SketchData>,
    cstate: &FootprintCanvasState,
    click_world: (f64, f64),
) -> Option<SketchEntityId>
```

## Visibility

- `pub(super)`

## Docstring

v0.13.3 — Hit-test Lines / Arcs / Circles (everything that isn't
a Point — Points are caught by `sketch_snap`). Returns the
nearest entity within `SKETCH_SNAP_RADIUS_PX`. Used by the
Select tool so the user can grab line / arc / circle entities,
not just Points.

## Source
Lines 21–98 in `crates/oxide-app/src/library/editor/footprint/canvas/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/library/editor/footprint/canvas/hit_test.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [screen_dist_to_segment_sq](/crates/oxide-app/src/library/editor/footprint/canvas/geometry/screen_dist_to_segment_sq.md) |
| called_by | [released_sketch_click](/crates/oxide-app/src/library/editor/footprint/canvas/input/release/released_sketch_click.md) |
