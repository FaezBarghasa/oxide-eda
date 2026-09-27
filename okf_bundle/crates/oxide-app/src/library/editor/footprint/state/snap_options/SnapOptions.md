---
okf_version: "0.2"
type: Class
title: SnapOptions
description: v0.17.0 — per-priority snap toggles surfaced on the empty-canvas
resource: crates/oxide-app/src/library/editor/footprint/state/snap_options.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/snap_options/SnapOptions
language: rust
---

# SnapOptions

v0.17.0 — per-priority snap toggles surfaced on the empty-canvas

## Signature

```rust
pub struct SnapOptions
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq)`

## Visibility

- `pub`

## Docstring

v0.17.0 — per-priority snap toggles surfaced on the empty-canvas
Properties panel. Mirrors Altium's "Snap Options" checklist.
[derive(Debug, Clone, Copy, PartialEq)]

## Methods

- `point_hit`
- `horizontal_vertical`
- `angle`
- `grid`
- `grid_step_mm`
- `fine_grid_display`
- `coarse_grid_display`
- `coarse_multiplier`
- `snap_track_vertices`
- `snap_track_lines`
- `snap_arc_centers`
- `snap_intersections`
- `snap_pad_centers`
- `snap_pad_vertices`
- `snap_pad_edges`
- `snap_via_centers`
- `snap_texts`
- `snap_regions`
- `snap_footprint_origins`
- `snap_3d_body_points`
- `snap_distance_mm`
- `axis_snap_range_mm`
- `snap_to_grids`
- `snap_to_guides`
- `snap_to_axes`

## Source
Lines 7–43 in `crates/oxide-app/src/library/editor/footprint/state/snap_options.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [snap_options](/crates/oxide-app/src/library/editor/footprint/state/snap_options.md) |
