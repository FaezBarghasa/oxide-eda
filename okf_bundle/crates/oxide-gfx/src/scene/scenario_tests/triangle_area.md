---
okf_version: "0.2"
type: Function
title: triangle_area
resource: crates/oxide-gfx/src/scene/scenario_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/scene/scenario_tests/triangle_area
language: rust
---

# triangle_area

## Signature

```rust
fn triangle_area(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f64
```

## Source
Lines 252–254 in `crates/oxide-gfx/src/scene/scenario_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scenario_tests](/crates/oxide-gfx/src/scene/scenario_tests.md) |
| calls | [shoelace_area](/crates/oxide-gfx/src/scene/scenario_tests/shoelace_area.md) |
| called_by | [concave_zone_fills_exactly_its_area](/crates/oxide-gfx/src/scene/scenario_tests/concave_zone_fills_exactly_its_area.md) |
