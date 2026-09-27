---
okf_version: "0.2"
type: Function
title: arc_sketch
description: "Sketch with one arc of radius `r_mm` centred at the origin, plus"
resource: crates/oxide-app/src/library/editor/footprint/canvas/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/tests/arc_sketch
language: rust
---

# arc_sketch

Sketch with one arc of radius `r_mm` centred at the origin, plus

## Signature

```rust
fn arc_sketch(r_mm: f64) -> (oxide_sketch::SketchData, oxide_sketch::id::SketchEntityId)
```

## Docstring

Sketch with one arc of radius `r_mm` centred at the origin, plus
its centre / start / end Points. Returns the arc's id.

## Source
Lines 103–134 in `crates/oxide-app/src/library/editor/footprint/canvas/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/canvas/tests.md) |
| called_by | [arc_hit_test_grabs_the_stroke_not_the_centre](/crates/oxide-app/src/library/editor/footprint/canvas/tests/arc_hit_test_grabs_the_stroke_not_the_centre.md) |
