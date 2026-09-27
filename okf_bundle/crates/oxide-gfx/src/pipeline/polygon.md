---
okf_version: "0.2"
type: Module
title: polygon
description: Polygon pipeline implementation.
resource: crates/oxide-gfx/src/pipeline/polygon.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/polygon
language: rust
---

# polygon

Polygon pipeline implementation.

## Docstring

Polygon pipeline implementation.

CLEAN ROOM DECLARATION
This module was written without reference to GPL-licensed software.
Sources: IPC-2612-1, IEEE 315, IEC 60617, wgpu/WGSL public docs.

## Relationships

| Type | Target |
|------|--------|
| related | [PolygonVertex](/crates/oxide-gfx/src/pipeline/polygon/PolygonVertex.md) |
| related | [triangulate_polygons](/crates/oxide-gfx/src/pipeline/polygon/triangulate_polygons.md) |
| related | [append_fill](/crates/oxide-gfx/src/pipeline/polygon/append_fill.md) |
| related | [append_fan_fill](/crates/oxide-gfx/src/pipeline/polygon/append_fan_fill.md) |
| related | [append_stroke](/crates/oxide-gfx/src/pipeline/polygon/append_stroke.md) |
| related | [append_edge_quad](/crates/oxide-gfx/src/pipeline/polygon/append_edge_quad.md) |
| related | [PolygonPipeline](/crates/oxide-gfx/src/pipeline/polygon/PolygonPipeline.md) |
| related | [new](/crates/oxide-gfx/src/pipeline/polygon/new.md) |
| related | [upload](/crates/oxide-gfx/src/pipeline/polygon/upload.md) |
| related | [upload_overlay](/crates/oxide-gfx/src/pipeline/polygon/upload_overlay.md) |
| related | [upload_into](/crates/oxide-gfx/src/pipeline/polygon/upload_into.md) |
| related | [draw](/crates/oxide-gfx/src/pipeline/polygon/draw.md) |
| related | [draw_overlay](/crates/oxide-gfx/src/pipeline/polygon/draw_overlay.md) |
| related | [draw_from](/crates/oxide-gfx/src/pipeline/polygon/draw_from.md) |
| related | [vertex_count](/crates/oxide-gfx/src/pipeline/polygon/vertex_count.md) |
| related | [new](/crates/oxide-gfx/src/pipeline/polygon/new.md) |
| related | [upload](/crates/oxide-gfx/src/pipeline/polygon/upload.md) |
| related | [upload_overlay](/crates/oxide-gfx/src/pipeline/polygon/upload_overlay.md) |
| related | [upload_into](/crates/oxide-gfx/src/pipeline/polygon/upload_into.md) |
| related | [draw](/crates/oxide-gfx/src/pipeline/polygon/draw.md) |
| related | [draw_overlay](/crates/oxide-gfx/src/pipeline/polygon/draw_overlay.md) |
| related | [draw_from](/crates/oxide-gfx/src/pipeline/polygon/draw_from.md) |
| related | [vertex_count](/crates/oxide-gfx/src/pipeline/polygon/vertex_count.md) |
| related | [shoelace_area](/crates/oxide-gfx/src/pipeline/polygon/shoelace_area.md) |
| related | [triangle_area](/crates/oxide-gfx/src/pipeline/polygon/triangle_area.md) |
| related | [triangulates_simple_convex_contour](/crates/oxide-gfx/src/pipeline/polygon/triangulates_simple_convex_contour.md) |
| related | [triangulates_every_contour_and_never_treats_it_as_triangle_soup](/crates/oxide-gfx/src/pipeline/polygon/triangulates_every_contour_and_never_treats_it_as_triangle_soup.md) |
| related | [concave_contour_fills_exactly_its_own_area](/crates/oxide-gfx/src/pipeline/polygon/concave_contour_fills_exactly_its_own_area.md) |
| related | [emits_a_stroke_outline_after_the_fill](/crates/oxide-gfx/src/pipeline/polygon/emits_a_stroke_outline_after_the_fill.md) |
| related | [no_stroke_when_color_is_absent](/crates/oxide-gfx/src/pipeline/polygon/no_stroke_when_color_is_absent.md) |
| related | [no_stroke_when_width_is_non_positive](/crates/oxide-gfx/src/pipeline/polygon/no_stroke_when_width_is_non_positive.md) |
| related | [skips_degenerate_polygons](/crates/oxide-gfx/src/pipeline/polygon/skips_degenerate_polygons.md) |
