---
okf_version: "0.2"
type: Function
title: unlink_corner_radius_mints_per_corner_param
description: v0.24 Phase 3 (Track A3) — dispatching FootprintSketchUnlinkCornerRadius
resource: crates/oxide-app/tests/regression/library_pad_geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_pad_geometry/unlink_corner_radius_mints_per_corner_param
language: rust
---

# unlink_corner_radius_mints_per_corner_param

v0.24 Phase 3 (Track A3) — dispatching FootprintSketchUnlinkCornerRadius

## Signature

```rust
fn unlink_corner_radius_mints_per_corner_param()
```

## Decorators

- `test`

## Docstring

v0.24 Phase 3 (Track A3) — dispatching FootprintSketchUnlinkCornerRadius
for one of the 4 corner Arcs mints a per-corner parameter and
records the override on `pad.shape_params`. The shared corner_r
binding stays in place so the other 3 corners follow it.
[test]

## Source
Lines 390–488 in `crates/oxide-app/tests/regression/library_pad_geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_pad_geometry](/crates/oxide-app/tests/regression/library_pad_geometry.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [SketchEntityId](/crates/oxide-sketch/src/id/SketchEntityId.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
