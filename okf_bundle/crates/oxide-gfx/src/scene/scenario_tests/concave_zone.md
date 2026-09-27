---
okf_version: "0.2"
type: Function
title: concave_zone
description: "A genuinely non-convex, 6-vertex notched contour standing in for a real"
resource: crates/oxide-gfx/src/scene/scenario_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/scene/scenario_tests/concave_zone
language: rust
---

# concave_zone

A genuinely non-convex, 6-vertex notched contour standing in for a real

## Signature

```rust
fn concave_zone() -> GpuPolygon
```

## Docstring

A genuinely non-convex, 6-vertex notched contour standing in for a real
copper pour/zone routed around an obstacle. Concave at `[12.0, 2.0]` (an
inward corner) — an L-shape, not a fan-friendly convex polygon.

## Source
Lines 75–89 in `crates/oxide-gfx/src/scene/scenario_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scenario_tests](/crates/oxide-gfx/src/scene/scenario_tests.md) |
| called_by | [concave_zone_fills_exactly_its_area](/crates/oxide-gfx/src/scene/scenario_tests/concave_zone_fills_exactly_its_area.md) |
