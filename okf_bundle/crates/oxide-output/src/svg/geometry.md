---
okf_version: "0.2"
type: Module
title: geometry
description: Low-level SVG path/shape primitives.
resource: crates/oxide-output/src/svg/geometry.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/geometry
language: rust
---

# geometry

Low-level SVG path/shape primitives.

## Docstring

Low-level SVG path/shape primitives.

Rectangle, circle, and three-point-arc path builders plus the
`tiny_skia` path conversion used by the rasterizer. These are the
geometry helpers the per-element emitters share.

Extracted verbatim from the SVG exporter (`svg/mod.rs`); pure code
motion, zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [path_to_tiny_skia](/crates/oxide-output/src/svg/geometry/path_to_tiny_skia.md) |
| related | [rect_path](/crates/oxide-output/src/svg/geometry/rect_path.md) |
| related | [circle_path](/crates/oxide-output/src/svg/geometry/circle_path.md) |
| related | [arc_path_commands](/crates/oxide-output/src/svg/geometry/arc_path_commands.md) |
| related | [circle_from_three_points](/crates/oxide-output/src/svg/geometry/circle_from_three_points.md) |
| related | [arc_sweep](/crates/oxide-output/src/svg/geometry/arc_sweep.md) |
