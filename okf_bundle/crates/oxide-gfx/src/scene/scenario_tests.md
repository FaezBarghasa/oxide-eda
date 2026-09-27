---
okf_version: "0.2"
type: Module
title: scenario_tests
description: Full-board scenario coverage for the CPU side of the GPU render path.
resource: crates/oxide-gfx/src/scene/scenario_tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/scene/scenario_tests
language: rust
---

# scenario_tests

Full-board scenario coverage for the CPU side of the GPU render path.

## Docstring

Full-board scenario coverage for the CPU side of the GPU render path.

CLEAN ROOM DECLARATION
This module was written without reference to GPL-licensed software.
Sources: IPC-2612-1, IEEE 315, IEC 60617, wgpu/WGSL public docs.

We cannot visually verify actual GPU shader pixel output without hardware,
so this module pins everything up to the GPU boundary instead: a single
[`Scene`] populated across *every* primitive bucket, the CPU-side
predicates (`is_dashed`/`is_filled`/`is_stroked`) that decide how each
primitive is meant to render, the CPU-side vertex generation
(`triangulate_polygons`) the GPU polygon pipeline uploads verbatim, and
the draw order both renderers are supposed to share. It deliberately does
NOT duplicate `order`'s own tests (the const-array parity checks and
`overlays_composite_above_base_buckets`) — it builds on top of them with
real, concretely-populated scene content.

## Relationships

| Type | Target |
|------|--------|
| related | [solid_line](/crates/oxide-gfx/src/scene/scenario_tests/solid_line.md) |
| related | [dashed_line](/crates/oxide-gfx/src/scene/scenario_tests/dashed_line.md) |
| related | [filled_circle](/crates/oxide-gfx/src/scene/scenario_tests/filled_circle.md) |
| related | [outline_circle](/crates/oxide-gfx/src/scene/scenario_tests/outline_circle.md) |
| related | [convex_filled_pad](/crates/oxide-gfx/src/scene/scenario_tests/convex_filled_pad.md) |
| related | [concave_zone](/crates/oxide-gfx/src/scene/scenario_tests/concave_zone.md) |
| related | [filled_and_stroked_polygon](/crates/oxide-gfx/src/scene/scenario_tests/filled_and_stroked_polygon.md) |
| related | [outline_only_rule_area](/crates/oxide-gfx/src/scene/scenario_tests/outline_only_rule_area.md) |
| related | [build_full_board_scene](/crates/oxide-gfx/src/scene/scenario_tests/build_full_board_scene.md) |
| related | [full_board_scenario_populates_every_bucket](/crates/oxide-gfx/src/scene/scenario_tests/full_board_scenario_populates_every_bucket.md) |
| related | [line_style_bit_is_preserved_in_the_scene_ir](/crates/oxide-gfx/src/scene/scenario_tests/line_style_bit_is_preserved_in_the_scene_ir.md) |
| related | [circle_and_polygon_predicates_split_correctly_across_the_scenario](/crates/oxide-gfx/src/scene/scenario_tests/circle_and_polygon_predicates_split_correctly_across_the_scenario.md) |
| related | [triangulate_convex_pad_fans_exactly_n_minus_2_fill_triangles](/crates/oxide-gfx/src/scene/scenario_tests/triangulate_convex_pad_fans_exactly_n_minus_2_fill_triangles.md) |
| related | [shoelace_area](/crates/oxide-gfx/src/scene/scenario_tests/shoelace_area.md) |
| related | [triangle_area](/crates/oxide-gfx/src/scene/scenario_tests/triangle_area.md) |
| related | [concave_zone_fills_exactly_its_area](/crates/oxide-gfx/src/scene/scenario_tests/concave_zone_fills_exactly_its_area.md) |
| related | [triangulate_filled_and_stroked_polygon_appends_stroke_after_fill](/crates/oxide-gfx/src/scene/scenario_tests/triangulate_filled_and_stroked_polygon_appends_stroke_after_fill.md) |
| related | [triangulate_outline_only_rule_area_still_emits_a_stroke](/crates/oxide-gfx/src/scene/scenario_tests/triangulate_outline_only_rule_area_still_emits_a_stroke.md) |
| related | [triangulate_the_full_scenario_polygon_batch_matches_the_per_polygon_sum](/crates/oxide-gfx/src/scene/scenario_tests/triangulate_the_full_scenario_polygon_batch_matches_the_per_polygon_sum.md) |
| related | [bucket_count](/crates/oxide-gfx/src/scene/scenario_tests/bucket_count.md) |
| related | [nonempty_bucket_sequence](/crates/oxide-gfx/src/scene/scenario_tests/nonempty_bucket_sequence.md) |
| related | [overlays_composite_above_base_content_in_a_fully_populated_scene](/crates/oxide-gfx/src/scene/scenario_tests/overlays_composite_above_base_content_in_a_fully_populated_scene.md) |
