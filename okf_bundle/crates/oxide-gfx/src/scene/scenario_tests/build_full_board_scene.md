---
okf_version: "0.2"
type: Function
title: build_full_board_scene
description: "One [`Scene`], every bucket populated: this is the shape a real board with"
resource: crates/oxide-gfx/src/scene/scenario_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/scene/scenario_tests/build_full_board_scene
language: rust
---

# build_full_board_scene

One [`Scene`], every bucket populated: this is the shape a real board with

## Signature

```rust
fn build_full_board_scene() -> Scene
```

## Docstring

One [`Scene`], every bucket populated: this is the shape a real board with
an active selection/highlight and ERC markers hands to both the CPU
canvas renderer and the GPU scene shader.

## Source
Lines 116–168 in `crates/oxide-gfx/src/scene/scenario_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scenario_tests](/crates/oxide-gfx/src/scene/scenario_tests.md) |
| called_by | [circle_and_polygon_predicates_split_correctly_across_the_scenario](/crates/oxide-gfx/src/scene/scenario_tests/circle_and_polygon_predicates_split_correctly_across_the_scenario.md) |
| called_by | [full_board_scenario_populates_every_bucket](/crates/oxide-gfx/src/scene/scenario_tests/full_board_scenario_populates_every_bucket.md) |
| called_by | [line_style_bit_is_preserved_in_the_scene_ir](/crates/oxide-gfx/src/scene/scenario_tests/line_style_bit_is_preserved_in_the_scene_ir.md) |
| called_by | [overlays_composite_above_base_content_in_a_fully_populated_scene](/crates/oxide-gfx/src/scene/scenario_tests/overlays_composite_above_base_content_in_a_fully_populated_scene.md) |
| called_by | [triangulate_the_full_scenario_polygon_batch_matches_the_per_polygon_sum](/crates/oxide-gfx/src/scene/scenario_tests/triangulate_the_full_scenario_polygon_batch_matches_the_per_polygon_sum.md) |
