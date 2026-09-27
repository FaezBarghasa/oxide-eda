---
okf_version: "0.2"
type: Module
title: geometry
description: Pure geometry helpers used by the canvas hit-test + draw passes.
resource: crates/oxide-app/src/library/editor/footprint/canvas/geometry.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/geometry
language: rust
---

# geometry

Pure geometry helpers used by the canvas hit-test + draw passes.

## Docstring

Pure geometry helpers used by the canvas hit-test + draw passes.
All free functions, all `pub(super)` so the surrounding canvas/
module can reach them.

## Relationships

| Type | Target |
|------|--------|
| related | [screen_dist_to_segment_sq](/crates/oxide-app/src/library/editor/footprint/canvas/geometry/screen_dist_to_segment_sq.md) |
| related | [point_to_segment_dist](/crates/oxide-app/src/library/editor/footprint/canvas/geometry/point_to_segment_dist.md) |
| related | [point_in_polygon](/crates/oxide-app/src/library/editor/footprint/canvas/geometry/point_in_polygon.md) |
| related | [polygon_outline_hit](/crates/oxide-app/src/library/editor/footprint/canvas/geometry/polygon_outline_hit.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
