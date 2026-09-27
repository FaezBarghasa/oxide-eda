---
okf_version: "0.2"
type: Function
title: triangulate_polygons
description: "Build the triangle-list vertices for a batch of polygons: an ear-clip fill"
resource: crates/oxide-gfx/src/pipeline/polygon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/polygon/triangulate_polygons
language: rust
---

# triangulate_polygons

Build the triangle-list vertices for a batch of polygons: an ear-clip fill

## Signature

```rust
pub(crate) fn triangulate_polygons(polygons: &[GpuPolygon]) -> Vec<PolygonVertex>
```

## Visibility

- `pub(crate)`

## Docstring

Build the triangle-list vertices for a batch of polygons: an ear-clip fill
for every contour, followed by a stroke outline (expanded edge quads) for
polygons that carry one. Fill and stroke share this pipeline's
`PolygonVertex` format, so they draw in a single pass — the stroke vertices
come last and composite on top of the fill under alpha blending, matching
the CPU `draw_polygons` order (fill, then stroke).

so misdrew any 3/6/9-vertex pour. Fill uses `oxide_sketch::ear_clip`
(below), which is exact for concave contours too — the CPU path
(`frame.fill`, lyon) tessellates the same contour, so the two now agree.

`pub(crate)` (rather than private) so `scene::scenario_tests`'s full-board
scenario can exercise this exact CPU-side vertex generation instead of
re-deriving the vertex-count formula independently.

## Source
Lines 31–45 in `crates/oxide-gfx/src/pipeline/polygon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polygon](/crates/oxide-gfx/src/pipeline/polygon.md) |
| calls | [append_fill](/crates/oxide-gfx/src/pipeline/polygon/append_fill.md) |
| calls | [append_stroke](/crates/oxide-gfx/src/pipeline/polygon/append_stroke.md) |
| called_by | [concave_contour_fills_exactly_its_own_area](/crates/oxide-gfx/src/pipeline/polygon/concave_contour_fills_exactly_its_own_area.md) |
| called_by | [emits_a_stroke_outline_after_the_fill](/crates/oxide-gfx/src/pipeline/polygon/emits_a_stroke_outline_after_the_fill.md) |
| called_by | [skips_degenerate_polygons](/crates/oxide-gfx/src/pipeline/polygon/skips_degenerate_polygons.md) |
| called_by | [triangulates_every_contour_and_never_treats_it_as_triangle_soup](/crates/oxide-gfx/src/pipeline/polygon/triangulates_every_contour_and_never_treats_it_as_triangle_soup.md) |
| called_by | [triangulates_simple_convex_contour](/crates/oxide-gfx/src/pipeline/polygon/triangulates_simple_convex_contour.md) |
| called_by | [upload](/crates/oxide-gfx/src/pipeline/polygon/upload.md) |
| called_by | [upload_overlay](/crates/oxide-gfx/src/pipeline/polygon/upload_overlay.md) |
| called_by | [concave_zone_fills_exactly_its_area](/crates/oxide-gfx/src/scene/scenario_tests/concave_zone_fills_exactly_its_area.md) |
| called_by | [triangulate_convex_pad_fans_exactly_n_minus_2_fill_triangles](/crates/oxide-gfx/src/scene/scenario_tests/triangulate_convex_pad_fans_exactly_n_minus_2_fill_triangles.md) |
| called_by | [triangulate_filled_and_stroked_polygon_appends_stroke_after_fill](/crates/oxide-gfx/src/scene/scenario_tests/triangulate_filled_and_stroked_polygon_appends_stroke_after_fill.md) |
| called_by | [triangulate_outline_only_rule_area_still_emits_a_stroke](/crates/oxide-gfx/src/scene/scenario_tests/triangulate_outline_only_rule_area_still_emits_a_stroke.md) |
| called_by | [triangulate_the_full_scenario_polygon_batch_matches_the_per_polygon_sum](/crates/oxide-gfx/src/scene/scenario_tests/triangulate_the_full_scenario_polygon_batch_matches_the_per_polygon_sum.md) |
