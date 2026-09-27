---
okf_version: "0.2"
type: Function
title: properties_panel_shows_corner_radius_for_round_rect_pad
description: v0.24 Phase 3 (Track A2) — placing a RoundRect pad in Pads mode
resource: crates/oxide-app/tests/regression/library_pad_geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_pad_geometry/properties_panel_shows_corner_radius_for_round_rect_pad
language: rust
---

# properties_panel_shows_corner_radius_for_round_rect_pad

v0.24 Phase 3 (Track A2) — placing a RoundRect pad in Pads mode

## Signature

```rust
fn properties_panel_shows_corner_radius_for_round_rect_pad()
```

## Decorators

- `test`

## Docstring

v0.24 Phase 3 (Track A2) — placing a RoundRect pad in Pads mode
registers a `corner_r` shape_params binding that the panel
context surfaces as a `PadShapeParamSummary` so the Properties
panel can render an editable "Corner radius" row.
[test]

## Source
Lines 233–307 in `crates/oxide-app/tests/regression/library_pad_geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_pad_geometry](/crates/oxide-app/tests/regression/library_pad_geometry.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
