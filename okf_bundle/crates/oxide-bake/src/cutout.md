---
okf_version: "0.2"
type: Module
title: cutout
description: Board cutout bake — turns BoardCutoutAttr-tagged closed profiles
resource: crates/oxide-bake/src/cutout.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/cutout
language: rust
---

# cutout

Board cutout bake — turns BoardCutoutAttr-tagged closed profiles

## Docstring

Board cutout bake — turns BoardCutoutAttr-tagged closed profiles
into `Footprint::cutouts: Vec<FpCutout>` records.

v0.14.1 records the polygon boundary; PCB outline subtraction
(the geometric op that combines cutouts with the board outline)
runs at PCB gerber-export time and is out of scope here.

v0.15 — `edge_radius_mm` (corner fillet radius) and `through`
(full-depth vs partial-depth) now propagate from
`BoardCutoutAttr` into `FpCutout`. The corner radius expression
is evaluated to mm via the parameter table; eval failure falls
back to `0.0` (sharp corner) with a warning.

## Relationships

| Type | Target |
|------|--------|
| related | [bake_cutouts](/crates/oxide-bake/src/cutout/bake_cutouts.md) |
| related | [build_ctx](/crates/oxide-bake/src/cutout/build_ctx.md) |
| related | [opt_eval_mm](/crates/oxide-bake/src/cutout/opt_eval_mm.md) |
| related | [solve](/crates/oxide-bake/src/cutout/solve.md) |
| related | [rectangle_with_cutout](/crates/oxide-bake/src/cutout/rectangle_with_cutout.md) |
| related | [bake_cutout_simple](/crates/oxide-bake/src/cutout/bake_cutout_simple.md) |
| related | [bake_cutout_edge_radius_evaluated](/crates/oxide-bake/src/cutout/bake_cutout_edge_radius_evaluated.md) |
| related | [bake_cutout_partial_depth_baked](/crates/oxide-bake/src/cutout/bake_cutout_partial_depth_baked.md) |
